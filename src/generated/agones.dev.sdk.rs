/// I am Empty
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}
/// Key, Value entry
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValue {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// time duration, in seconds
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Duration {
    #[prost(int64, tag = "1")]
    pub seconds: i64,
}
/// A GameServer Custom Resource Definition object
/// We will only export those resources that make the most
/// sense. Can always expand to more as needed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameServer {
    #[prost(message, optional, tag = "1")]
    pub object_meta: ::core::option::Option<game_server::ObjectMeta>,
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<game_server::Spec>,
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<game_server::Status>,
}
/// Nested message and enum types in `GameServer`.
pub mod game_server {
    /// representation of the K8s ObjectMeta resource
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ObjectMeta {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub namespace: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub uid: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub resource_version: ::prost::alloc::string::String,
        #[prost(int64, tag = "5")]
        pub generation: i64,
        /// timestamp is in Epoch format, unit: seconds
        #[prost(int64, tag = "6")]
        pub creation_timestamp: i64,
        /// optional deletion timestamp in Epoch format, unit: seconds
        #[prost(int64, tag = "7")]
        pub deletion_timestamp: i64,
        #[prost(map = "string, string", tag = "8")]
        pub annotations: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
        #[prost(map = "string, string", tag = "9")]
        pub labels: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Spec {
        #[prost(message, optional, tag = "1")]
        pub health: ::core::option::Option<spec::Health>,
    }
    /// Nested message and enum types in `Spec`.
    pub mod spec {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Health {
            #[prost(bool, tag = "1")]
            pub disabled: bool,
            #[prost(int32, tag = "2")]
            pub period_seconds: i32,
            #[prost(int32, tag = "3")]
            pub failure_threshold: i32,
            #[prost(int32, tag = "4")]
            pub initial_delay_seconds: i32,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Status {
        #[prost(string, tag = "1")]
        pub state: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub address: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "3")]
        pub ports: ::prost::alloc::vec::Vec<status::Port>,
        /// \[Stage:Alpha\]
        /// \[FeatureFlag:PlayerTracking\]
        #[prost(message, optional, tag = "4")]
        pub players: ::core::option::Option<status::PlayerStatus>,
    }
    /// Nested message and enum types in `Status`.
    pub mod status {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Port {
            #[prost(string, tag = "1")]
            pub name: ::prost::alloc::string::String,
            #[prost(int32, tag = "2")]
            pub port: i32,
        }
        /// \[Stage:Alpha\]
        /// \[FeatureFlag:PlayerTracking\]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PlayerStatus {
            #[prost(int64, tag = "1")]
            pub count: i64,
            #[prost(int64, tag = "2")]
            pub capacity: i64,
            #[prost(string, repeated, tag = "3")]
            pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        }
    }
}
/// Generated client implementations.
pub mod sdk_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// SDK service to be used in the GameServer SDK to the Pod Sidecar
    #[derive(Debug, Clone)]
    pub struct SdkClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SdkClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SdkClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
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
        /// Call when the GameServer is ready
        pub async fn ready(
            &mut self,
            request: impl tonic::IntoRequest<super::Empty>,
        ) -> Result<tonic::Response<super::Empty>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/agones.dev.sdk.SDK/Ready");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Call to self Allocation the GameServer
        pub async fn allocate(
            &mut self,
            request: impl tonic::IntoRequest<super::Empty>,
        ) -> Result<tonic::Response<super::Empty>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agones.dev.sdk.SDK/Allocate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Call when the GameServer is shutting down
        pub async fn shutdown(
            &mut self,
            request: impl tonic::IntoRequest<super::Empty>,
        ) -> Result<tonic::Response<super::Empty>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agones.dev.sdk.SDK/Shutdown",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Send a Empty every d Duration to declare that this GameSever is healthy
        pub async fn health(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::Empty>,
        ) -> Result<tonic::Response<super::Empty>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agones.dev.sdk.SDK/Health",
            );
            self.inner
                .client_streaming(request.into_streaming_request(), path, codec)
                .await
        }
        /// Retrieve the current GameServer data
        pub async fn get_game_server(
            &mut self,
            request: impl tonic::IntoRequest<super::Empty>,
        ) -> Result<tonic::Response<super::GameServer>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agones.dev.sdk.SDK/GetGameServer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Send GameServer details whenever the GameServer is updated
        pub async fn watch_game_server(
            &mut self,
            request: impl tonic::IntoRequest<super::Empty>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::GameServer>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agones.dev.sdk.SDK/WatchGameServer",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Apply a Label to the backing GameServer metadata
        pub async fn set_label(
            &mut self,
            request: impl tonic::IntoRequest<super::KeyValue>,
        ) -> Result<tonic::Response<super::Empty>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agones.dev.sdk.SDK/SetLabel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Apply a Annotation to the backing GameServer metadata
        pub async fn set_annotation(
            &mut self,
            request: impl tonic::IntoRequest<super::KeyValue>,
        ) -> Result<tonic::Response<super::Empty>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agones.dev.sdk.SDK/SetAnnotation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Marks the GameServer as the Reserved state for Duration
        pub async fn reserve(
            &mut self,
            request: impl tonic::IntoRequest<super::Duration>,
        ) -> Result<tonic::Response<super::Empty>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agones.dev.sdk.SDK/Reserve",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}