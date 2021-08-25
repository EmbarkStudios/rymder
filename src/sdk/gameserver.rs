use crate::proto::api::{self, game_server};

/// Different exclusive states a `GameServer` can be in. See the
/// [docs](https://agones.dev/site/docs/guides/client-sdks/#function-reference)
/// for more information
#[derive(Copy, Clone)]
pub enum State {
    /// [Ready](https://agones.dev/site/docs/guides/client-sdks/#ready) to take
    /// player connections
    Ready,
    /// [Reserved](https://agones.dev/site/docs/guides/client-sdks/#reserveseconds)
    /// so the `GameServer` can't be deleted, but doesn't trigger a Fleet scaleup
    Reserved,
    /// [Allocated](https://agones.dev/site/docs/guides/client-sdks/#allocate)
    /// means the `GameServer` has active players and should not be deleted or
    /// scaled down
    Allocated,
    /// [Shutdown](https://agones.dev/site/docs/guides/client-sdks/#shutdown)
    /// marks the `GameServer` as reapable
    Shutdown,
}

pub struct Port {
    pub name: String,
    pub port: u16,
}

pub struct Status {
    pub state: State,
    pub address: std::net::IpAddr,
    pub ports: Vec<Port>,
    #[cfg(feature = "player-tracking")]
    pub players: api::PlayerStatus,
}

/// A strongly typed wrapper around the generated [`GameServer`](crate::proto::api::GameServer).
pub struct GameServer {
    pub object_meta: game_server::ObjectMeta,
    pub spec: game_server::Spec,
    pub status: Status,
}

impl From<api::GameServer> for GameServer {
    fn from(ogs: api::GameServer) -> Self {
        unimplemented!()
    }
}
