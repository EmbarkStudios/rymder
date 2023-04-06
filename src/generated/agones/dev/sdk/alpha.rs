/// I am Empty
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}
/// Store a count variable.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Count {
    #[prost(int64, tag = "1")]
    pub count: i64,
}
/// Store a boolean result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bool {
    #[prost(bool, tag = "1")]
    pub bool: bool,
}
/// The unique identifier for a given player.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerId {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
}
/// List of Player IDs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerIdList {
    #[prost(string, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Generated client implementations.
pub mod sdk_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// SDK service to be used in the GameServer SDK to the Pod Sidecar.
    #[derive(Debug, Clone)]
    pub struct SdkClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SdkClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> SdkClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            SdkClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// PlayerConnect increases the SDK’s stored player count by one, and appends this playerID to GameServer.Status.Players.IDs.
        ///
        /// GameServer.Status.Players.Count and GameServer.Status.Players.IDs are then set to update the player count and id list a second from now,
        /// unless there is already an update pending, in which case the update joins that batch operation.
        ///
        /// PlayerConnect returns true and adds the playerID to the list of playerIDs if this playerID was not already in the
        /// list of connected playerIDs.
        ///
        /// If the playerID exists within the list of connected playerIDs, PlayerConnect will return false, and the list of
        /// connected playerIDs will be left unchanged.
        ///
        /// An error will be returned if the playerID was not already in the list of connected playerIDs but the player capacity for
        /// the server has been reached. The playerID will not be added to the list of playerIDs.
        ///
        /// Warning: Do not use this method if you are manually managing GameServer.Status.Players.IDs and GameServer.Status.Players.Count
        /// through the Kubernetes API, as indeterminate results will occur.
        pub async fn player_connect(
            &mut self,
            request: impl tonic::IntoRequest<super::PlayerId>,
        ) -> std::result::Result<tonic::Response<super::Bool>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/agones.dev.sdk.alpha.SDK/PlayerConnect");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("agones.dev.sdk.alpha.SDK", "PlayerConnect"));
            self.inner.unary(req, path, codec).await
        }
        /// Decreases the SDK’s stored player count by one, and removes the playerID from GameServer.Status.Players.IDs.
        ///
        /// GameServer.Status.Players.Count and GameServer.Status.Players.IDs are then set to update the player count and id list a second from now,
        /// unless there is already an update pending, in which case the update joins that batch operation.
        ///
        /// PlayerDisconnect will return true and remove the supplied playerID from the list of connected playerIDs if the
        /// playerID value exists within the list.
        ///
        /// If the playerID was not in the list of connected playerIDs, the call will return false, and the connected playerID list
        /// will be left unchanged.
        ///
        /// Warning: Do not use this method if you are manually managing GameServer.status.players.IDs and GameServer.status.players.Count
        /// through the Kubernetes API, as indeterminate results will occur.
        pub async fn player_disconnect(
            &mut self,
            request: impl tonic::IntoRequest<super::PlayerId>,
        ) -> std::result::Result<tonic::Response<super::Bool>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/agones.dev.sdk.alpha.SDK/PlayerDisconnect");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "agones.dev.sdk.alpha.SDK",
                "PlayerDisconnect",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// Update the GameServer.Status.Players.Capacity value with a new capacity.
        pub async fn set_player_capacity(
            &mut self,
            request: impl tonic::IntoRequest<super::Count>,
        ) -> std::result::Result<tonic::Response<super::Empty>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/agones.dev.sdk.alpha.SDK/SetPlayerCapacity");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "agones.dev.sdk.alpha.SDK",
                "SetPlayerCapacity",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves the current player capacity. This is always accurate from what has been set through this SDK,
        /// even if the value has yet to be updated on the GameServer status resource.
        ///
        /// If GameServer.Status.Players.Capacity is set manually through the Kubernetes API, use SDK.GameServer() or SDK.WatchGameServer() instead to view this value.
        pub async fn get_player_capacity(
            &mut self,
            request: impl tonic::IntoRequest<super::Empty>,
        ) -> std::result::Result<tonic::Response<super::Count>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/agones.dev.sdk.alpha.SDK/GetPlayerCapacity");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "agones.dev.sdk.alpha.SDK",
                "GetPlayerCapacity",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves the current player count. This is always accurate from what has been set through this SDK,
        /// even if the value has yet to be updated on the GameServer status resource.
        ///
        /// If GameServer.Status.Players.Count is set manually through the Kubernetes API, use SDK.GameServer() or SDK.WatchGameServer() instead to view this value.
        pub async fn get_player_count(
            &mut self,
            request: impl tonic::IntoRequest<super::Empty>,
        ) -> std::result::Result<tonic::Response<super::Count>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/agones.dev.sdk.alpha.SDK/GetPlayerCount");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "agones.dev.sdk.alpha.SDK",
                "GetPlayerCount",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// Returns if the playerID is currently connected to the GameServer. This is always accurate from what has been set through this SDK,
        /// even if the value has yet to be updated on the GameServer status resource.
        ///
        /// If GameServer.Status.Players.IDs is set manually through the Kubernetes API, use SDK.GameServer() or SDK.WatchGameServer() instead to determine connected status.
        pub async fn is_player_connected(
            &mut self,
            request: impl tonic::IntoRequest<super::PlayerId>,
        ) -> std::result::Result<tonic::Response<super::Bool>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/agones.dev.sdk.alpha.SDK/IsPlayerConnected");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "agones.dev.sdk.alpha.SDK",
                "IsPlayerConnected",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// Returns the list of the currently connected player ids. This is always accurate from what has been set through this SDK,
        /// even if the value has yet to be updated on the GameServer status resource.
        ///
        /// If GameServer.Status.Players.IDs is set manually through the Kubernetes API, use SDK.GameServer() or SDK.WatchGameServer() instead to view this value.
        pub async fn get_connected_players(
            &mut self,
            request: impl tonic::IntoRequest<super::Empty>,
        ) -> std::result::Result<tonic::Response<super::PlayerIdList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agones.dev.sdk.alpha.SDK/GetConnectedPlayers",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "agones.dev.sdk.alpha.SDK",
                "GetConnectedPlayers",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
