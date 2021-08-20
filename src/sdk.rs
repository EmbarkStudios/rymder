use std::{env, time::Duration};
use tonic::transport::Channel;

use crate::proto::api::{self, sdk_client::SdkClient};

#[cfg(feature = "player-tracking")]
use crate::proto::alpha::{self, sdk_client::SdkClient as AlphaClient};

pub use api::GameServer;

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
    /// Starts a new SDK instance, and connects to localhost on the port specified
    /// or else falls back to the `AGONES_SDK_GRPC_PORT` environment variable,
    /// or defaults to 9357.
    ///
    /// # Errors
    ///
    /// - The port specified in `AGONES_SDK_GRPC_PORT` can't be parsed as a `u16`.
    /// - A connection cannot be established with an Agones SDK server
    /// - The handshake takes longer than the specified timeout duration
    pub async fn new(
        port: Option<u16>,
        timeout: Option<Duration>,
        keep_alive: Option<Duration>,
    ) -> Result<Self> {
        let addr: http::Uri = format!(
            "http://localhost:{}",
            port.unwrap_or_else(|| {
                env::var("AGONES_SDK_GRPC_PORT")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(9357)
            })
        )
        .parse()?;

        let builder = tonic::transport::channel::Channel::builder(addr)
            .keep_alive_timeout(keep_alive.unwrap_or_else(|| Duration::from_secs(30)));

        let channel = builder.connect().await?;
        let mut client = SdkClient::new(channel.clone());

        #[cfg(feature = "player-tracking")]
        let alpha = AlphaClient::new(channel);

        tokio::time::timeout(timeout.unwrap_or_else(|| Duration::from_secs(30)), async {
            let mut connect_interval = tokio::time::interval(Duration::from_millis(100));

            loop {
                connect_interval.tick().await;

                if client.get_game_server(empty()).await.is_ok() {
                    break;
                }
            }
        })
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

    /// Returns a [`tokio::sync::mpsc::Sender`] that will emit a health check
    /// every time a message is sent on the channel.
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

    /// Set a Label value on the backing Game Server record that is stored in Kubernetes
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

    /// Set a Annotation value on the backing Game Server record that is stored in Kubernetes
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
            .map(|res| res.into_inner())?)
    }

    /// Reserve marks the Game Server as Reserved for a given duration, at which
    /// point it will return the Game Server to a Ready state.
    /// Note that the smallest reserve duration is 1 second.
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
    /// this playerID to `GameServer.status.players.ids`.
    ///
    /// Returns true and adds the playerID to the list of playerIDs if the
    /// playerIDs was not already in the list of connected playerIDs.
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
    /// the playerID from GameServer.status.players.ids.
    ///
    /// Will return true and remove the supplied playerID from the list of
    /// connected playerIDs if the playerID value exists within the list.
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

    /// This returns if the playerID is currently connected to the GameServer.
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

    /// This returns the list of the currently connected player ids.
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
