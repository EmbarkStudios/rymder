//! Wrappers around various types generated from protobuf definitions to make
//! them more ergonomic

use chrono::TimeZone;

use crate::{proto::api, Error};
use std::time::Duration;

/// Different exclusive states a `GameServer` can be in. See the
/// [docs](https://agones.dev/site/docs/guides/client-sdks/#function-reference)
/// for more information.
///
/// The list of possible states comes from [here](https://github.com/googleforgames/agones/blob/57005f77f6fdb619d856acf4e434810c2ab59c1b/pkg/apis/agones/v1/gameserver.go#L35-L62)
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
    /// When the `GameServer` is being allocated dynamically, and an open port
    /// needs to be allocated. It is unlikely/impossible for this state to be
    /// observed from the gameserver itself.
    PortAllocation,
    /// Before the k8s pod is created. Again, unlikely/impossible for this state
    /// to be observed from the gameserver itself.
    Creating,
    /// The k8s pod for the `GameServer` is being created. Again, unlikely/impossible
    /// for this state to be observed from the gameserver itself.
    Starting,
    /// The initial state of a newly created `GameServer` pod. Note this state
    /// is not sent by the SDK server when used locally.
    Scheduled,
    /// The `GameServer` has declared itself ready
    RequestReady,
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
    /// Failed to send health checks in a timely manner according to the health
    /// spec assigned to the `GameServer`
    Unhealthy,
    /// [Shutdown](https://agones.dev/site/docs/guides/client-sdks/#shutdown)
    /// marks the `GameServer` as reapable
    Shutdown,
    /// Something has gone wrong that cannot be resolved
    Error,
}

impl std::str::FromStr for State {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "PortAllocation" => Self::PortAllocation,
            "Creating" => Self::Creating,
            "Starting" => Self::Starting,
            "Scheduled" => Self::Scheduled,
            "RequestReady" => Self::RequestReady,
            "Ready" => Self::Ready,
            "Reserved" => Self::Reserved,
            "Allocated" => Self::Allocated,
            "Unhealthy" => Self::Unhealthy,
            "Shutdown" => Self::Shutdown,
            "Error" => Self::Error,
            unknown_state => return Err(Error::UnknownState(unknown_state.to_owned())),
        })
    }
}

/// A port exposed by the container
#[derive(Debug)]
pub struct Port {
    /// The name of the port
    pub name: String,
    /// The actual port number
    pub port: u16,
}

/// A more strongly-typed wrapper around
/// [`Status`](crate::proto::api::game_server::Status)
#[derive(Debug)]
pub struct Status {
    /// The current state of the `GameServer`, see [Lifecycle Management](
    /// https://agones.dev/site/docs/guides/client-sdks/#lifecycle-management)
    /// for more details
    pub state: State,
    /// The pubic IP address the `GameServer` is being served from
    pub address: std::net::IpAddr,
    /// The ports exposed by the `GameServer` container
    pub ports: Vec<Port>,
    /// The current number, capacity, and list of connected player identifiers
    #[cfg(feature = "player-tracking")]
    pub players: Option<api::game_server::status::PlayerStatus>,
}

/// Representation of the k8s
/// [`ObjectMeta`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.19/#objectmeta-v1-meta)
/// resource
#[derive(Debug)]
pub struct ObjectMeta {
    /// The name of the pod in k8s
    pub name: String,
    /// The namespace in k8s the pod is running in
    pub namespace: String,
    /// The [uuid](https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids)
    /// assigned to the pod
    pub uid: String,
    /// The k8s [resource version](https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions)
    /// for the pod
    pub resource_version: String,
    /// The [generation](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/#status-subresource)
    /// of the deployed pod
    pub generation: i64,
    /// The time the pod was [created](https://kubernetes.io/docs/reference/using-api/api-concepts/#generated-values)
    pub creation_timestamp: chrono::DateTime<chrono::Utc>,
    /// The time the pod was [deleted](https://kubernetes.io/docs/reference/using-api/api-concepts/#generated-values)
    pub deletion_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    /// The [annotations](https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/)
    /// currently applied to the pod
    pub annotations: std::collections::HashMap<String, String>,
    /// The [labels](https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/)
    /// currently applied to the pod
    pub labels: std::collections::HashMap<String, String>,
}

/// More strongly-typed wrapper around
/// [`Health`](crate::proto::api::game_server::spec::Health)
#[derive(Debug)]
pub struct HealthSpec {
    /// Interval at which health checks must be sent for the gameserver to be
    /// considered healthy
    pub period: Duration,
    /// Minimum number of consecutive failures for the health probe to be
    /// considered failed
    pub failure_threshold: std::num::NonZeroU32,
    /// Time after the gameserver has started before the health check is started
    pub initial_delay: Duration,
}

/// A strongly typed wrapper around the generated
/// [`GameServer`](crate::proto::api::GameServer).
#[derive(Debug)]
pub struct GameServer {
    /// k8s object metadata
    pub object_meta: Option<ObjectMeta>,
    /// Currently, health is the
    /// [only item](crate::proto::api::game_server::Spec::health) exposed from
    /// the [Spec](crate::proto::api::GameServer::spec), so it is just made into
    /// a top level field here. This is `None` if the either `spec` or
    /// `spec.health` is `None` in the original `GameServer`, or if
    /// `spec.health.disabled == true`.
    pub health_spec: Option<HealthSpec>,
    /// State information
    pub status: Option<Status>,
}

impl std::convert::TryFrom<api::GameServer> for GameServer {
    type Error = Error;

    fn try_from(ogs: api::GameServer) -> Result<Self, Self::Error> {
        let status = match ogs.status {
            Some(status) => {
                let address = match status.address.parse() {
                    Ok(addr) => addr,
                    Err(err) => {
                        return Err(Error::InvalidIp {
                            ip_str: status.address,
                            err,
                        });
                    }
                };

                let state = status.state.parse()?;

                Some(Status {
                    state,
                    address,
                    // We _could_ error on invalid ports that aren't in the u16 range, but
                    // it feels like if agones is sending those something even worse
                    // is going to happen
                    ports: status
                        .ports
                        .into_iter()
                        .map(|port| Port {
                            name: port.name,
                            port: port.port as u16,
                        })
                        .collect(),
                    #[cfg(feature = "player-tracking")]
                    players: status.players,
                })
            }
            None => None,
        };

        let object_meta = ogs.object_meta.map(|om| {
            let dt = om.deletion_timestamp;

            ObjectMeta {
                name: om.name,
                namespace: om.namespace,
                uid: om.uid,
                resource_version: om.resource_version,
                generation: om.generation,
                creation_timestamp: chrono::Utc.timestamp(om.creation_timestamp, 0),
                deletion_timestamp: (dt != 0).then(|| chrono::Utc.timestamp(dt, 0)),
                annotations: om.annotations,
                labels: om.labels,
            }
        });

        let health_spec = ogs.spec.and_then(|spec| {
            spec.health.and_then(|health| {
                if health.disabled {
                    None
                } else {
                    std::num::NonZeroU32::new(health.failure_threshold as u32).map(
                        |failure_threshold| HealthSpec {
                            period: Duration::from_secs(health.period_seconds as u64),
                            failure_threshold,
                            initial_delay: Duration::from_secs(health.initial_delay_seconds as u64),
                        },
                    )
                }
            })
        });

        Ok(Self {
            object_meta,
            health_spec,
            status,
        })
    }
}

#[cfg(test)]
mod test {
    use super::State;

    #[test]
    fn string_states() {
        assert_eq!(
            "PortAllocation".parse::<State>().unwrap(),
            State::PortAllocation
        );
        assert_eq!("Creating".parse::<State>().unwrap(), State::Creating);
        assert_eq!("Starting".parse::<State>().unwrap(), State::Starting);
        assert_eq!("Scheduled".parse::<State>().unwrap(), State::Scheduled);
        assert_eq!(
            "RequestReady".parse::<State>().unwrap(),
            State::RequestReady
        );
        assert_eq!("Ready".parse::<State>().unwrap(), State::Ready);
        assert_eq!("Shutdown".parse::<State>().unwrap(), State::Shutdown);
        assert_eq!("Error".parse::<State>().unwrap(), State::Error);
        assert_eq!("Unhealthy".parse::<State>().unwrap(), State::Unhealthy);
        assert_eq!("Reserved".parse::<State>().unwrap(), State::Reserved);
        assert_eq!("Allocated".parse::<State>().unwrap(), State::Allocated);
    }
}
