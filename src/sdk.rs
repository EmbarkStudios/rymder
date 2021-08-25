mod gameserver;

use std::{env, time::Duration};
use tonic::transport::Channel;

use crate::proto::api::{self, sdk_client::SdkClient};

#[cfg(feature = "player-tracking")]
use crate::proto::alpha::{self, sdk_client::SdkClient as AlphaClient};

pub use gameserver::GameServer;

pub type WatchStream = tonic::Streaming<GameServer>;

use crate::errors::Result;

#[inline]
fn empty() -> api::Empty {
    api::Empty {}
}

/// SDK is an instance of the Agones SDK
#[derive(Clone)]
pub struct Sdk {
    client: SdkClient<Channel>,
    #[cfg(feature = "player-tracking")]
    alpha: AlphaClient<Channel>,
}

impl Sdk {
    /// Starts a new SDK instance, and connects to localhost on the `port` specified
    /// or else falls back to the `AGONES_SDK_GRPC_PORT` environment variable,
    /// or defaults to 9357.
    ///
    /// The `handshake_timeout` applies to the time it takes to perform the
    /// initial handshake with the agones sidecar once a connection has been
    /// established.
    ///
    /// # Errors
    ///
    /// - The port specified in `AGONES_SDK_GRPC_PORT` can't be parsed as a `u16`.
    /// - A connection cannot be established with an Agones SDK server
    /// - The handshake takes longer than the specified `handshake_timeout` duration
    pub async fn new(
        port: Option<u16>,
        handshake_timeout: Option<Duration>,
        keep_alive: Option<Duration>,
    ) -> Result<Self> {
        let addr: http::Uri = format!(
            "http://localhost:{}",
            match port {
                Some(port) => port,
                None => {
                    match env::var("AGONES_SDK_GRPC_PORT") {
                        Ok(val) => val.parse().map_err(crate::Error::ParseInteger)?,
                        Err(_) => 9357,
                    }
                }
            }
        )
        .parse()?;

        let builder = tonic::transport::channel::Channel::builder(addr)
            .keep_alive_timeout(keep_alive.unwrap_or_else(|| Duration::from_secs(30)));

        let channel = builder.connect().await?;
        let mut client = SdkClient::new(channel.clone());

        #[cfg(feature = "player-tracking")]
        let alpha = AlphaClient::new(channel);

        tokio::time::timeout(
            handshake_timeout.unwrap_or_else(|| Duration::from_secs(30)),
            async {
                let mut connect_interval = tokio::time::interval(Duration::from_millis(100));

                loop {
                    connect_interval.tick().await;

                    if client.get_game_server(empty()).await.is_ok() {
                        break;
                    }
                }
            },
        )
        .await?;

        Ok(Self {
            client,
            #[cfg(feature = "player-tracking")]
            alpha,
        })
    }

    /// Marks the Game Server as ready to receive connections
    #[inline]
    pub async fn mark_ready(&mut self) -> Result<()> {
        Ok(self.client.ready(empty()).await.map(|_| ())?)
    }

    /// Allocate the Game Server
    #[inline]
    pub async fn allocate(&mut self) -> Result<()> {
        Ok(self.client.allocate(empty()).await.map(|_| ())?)
    }

    /// Marks the Game Server as ready to shutdown
    #[inline]
    pub async fn shutdown(&mut self) -> Result<()> {
        Ok(self.client.shutdown(empty()).await.map(|_| ())?)
    }

    /// Returns a [`tokio::sync::mpsc::Sender`](https://docs.rs/tokio/1.10.0/tokio/sync/mpsc/struct.Sender.html)
    /// that will emit a health check every time a message is sent on the channel.
    pub fn health_check(&self) -> tokio::sync::mpsc::Sender<()> {
        let mut health_client = self.clone();
        let (tx, mut rx) = tokio::sync::mpsc::channel(10);

        tokio::task::spawn(async move {
            let health_stream = async_stream::stream! {
                while rx.recv().await.is_some() {
                    yield empty();
                }
            };

            let _ = health_client.client.health(health_stream).await;
        });

        tx
    }

    /// Set a [Label](https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/)
    /// value on the backing Game Server record that is stored in Kubernetes
    #[inline]
    pub async fn set_label(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<()> {
        Ok(self
            .client
            .set_label(api::KeyValue {
                key: key.into(),
                value: value.into(),
            })
            .await
            .map(|_| ())?)
    }

    /// Set an [Annotation](https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/)
    /// value on the backing Game Server record that is stored in Kubernetes
    #[inline]
    pub async fn set_annotation(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<()> {
        Ok(self
            .client
            .set_annotation(api::KeyValue {
                key: key.into(),
                value: value.into(),
            })
            .await
            .map(|_| ())?)
    }

    /// Returns most of the backing Game Server configuration and Status
    #[inline]
    pub async fn get_gameserver(&mut self) -> Result<GameServer> {
        Ok(self
            .client
            .get_game_server(empty())
            .await
            .map(|res| res.into_inner().into())?)
    }

    /// Reserve marks the Game Server as Reserved for a given duration, at which
    /// point it will return the Game Server to a Ready state.
    ///
    /// Note that the smallest reserve duration is 1 second and is limited to
    /// second resolution.
    #[inline]
    pub async fn reserve(&mut self, duration: Duration) -> Result<()> {
        Ok(self
            .client
            .reserve(api::Duration {
                seconds: std::cmp::max(duration.as_secs() as i64, 1),
            })
            .await
            .map(|_| ())?)
    }

    /// Watch the backing Game Server configuration on updated
    pub async fn watch_gameserver(&mut self) -> Result<WatchStream> {
        Ok(self
            .client
            .watch_game_server(empty())
            .await
            .map(|stream| stream.into_inner())?)
    }
}

#[cfg(feature = "player-tracking")]
impl Sdk {
    /// This returns the last player capacity that was set through the SDK.
    /// If the player capacity is set from outside the SDK, use
    /// [`Sdk::get_gameserver`] instead.
    #[inline]
    pub async fn get_player_capacity(&mut self) -> Result<i64> {
        Ok(self
            .alpha
            .get_player_capacity(alpha::Empty {})
            .await
            .map(|c| c.into_inner().count)?)
    }

    /// This changes the player capacity to a new value.
    #[inline]
    pub async fn set_player_capacity(&mut self, count: i64) -> Result<()> {
        Ok(self
            .alpha
            .set_player_capacity(alpha::Count { count })
            .await
            .map(|_| ())?)
    }

    /// This function increases the SDK’s stored player count by one, and appends
    /// this player id to `GameServer.status.players.ids`.
    ///
    /// Returns true and adds the player id to the list of player ids if it
    /// was not already present.
    #[inline]
    pub async fn player_connect(&mut self, id: impl Into<String>) -> Result<bool> {
        Ok(self
            .alpha
            .player_connect(alpha::PlayerId {
                player_id: id.into(),
            })
            .await
            .map(|b| b.into_inner().bool)?)
    }

    /// This function decreases the SDK’s stored player count by one, and removes
    /// the player id from `GameServer.status.players.ids`.
    ///
    /// Will return true and remove the supplied player id from the list of
    /// connected player ids if the player id exists within the list.
    #[inline]
    pub async fn player_disconnect(&mut self, id: impl Into<String>) -> Result<bool> {
        Ok(self
            .alpha
            .player_disconnect(alpha::PlayerId {
                player_id: id.into(),
            })
            .await
            .map(|b| b.into_inner().bool)?)
    }

    /// Returns the current player count.
    #[inline]
    pub async fn get_player_count(&mut self) -> Result<i64> {
        Ok(self
            .alpha
            .get_player_count(alpha::Empty {})
            .await
            .map(|c| c.into_inner().count)?)
    }

    /// Returns whether the player id is currently connected to the Game Server.
    /// This is always accurate, even if the value hasn’t been updated to the
    /// Game Server status yet.
    #[inline]
    pub async fn is_player_connected(&mut self, id: impl Into<String>) -> Result<bool> {
        Ok(self
            .alpha
            .is_player_connected(alpha::PlayerId {
                player_id: id.into(),
            })
            .await
            .map(|b| b.into_inner().bool)?)
    }

    /// Returns the list of the currently connected player ids.
    /// This is always accurate, even if the value has not been updated to the
    /// Game Server status yet.
    #[inline]
    pub async fn get_connected_players(&mut self) -> Result<Vec<String>> {
        Ok(self
            .alpha
            .get_connected_players(alpha::Empty {})
            .await
            .map(|pl| pl.into_inner().list)?)
    }
}
