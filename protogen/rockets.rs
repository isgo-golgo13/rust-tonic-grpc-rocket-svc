#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareRocketLaunchSeqRequest {
    #[prost(string, tag = "1")]
    pub pre_launch_seq: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareRocketLaunchSeqResponse {
    #[prost(string, tag = "1")]
    pub pre_launch_resp: ::prost::alloc::string::String,
}
/// argument
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RocketLaunchRequest {
    /// data type and position of data
    #[prost(string, tag = "1")]
    pub launch_req: ::prost::alloc::string::String,
}
/// return value
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RocketLaunchResponse {
    /// data type and position of data
    #[prost(string, tag = "1")]
    pub launch_resp: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod rocket_launcher_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " service which can be executed"]
    #[derive(Debug, Clone)]
    pub struct RocketLauncherClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RocketLauncherClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> RocketLauncherClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> RocketLauncherClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            RocketLauncherClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " function which can be called"]
        #[doc = "Pre-launch API"]
        pub async fn send_pre_launch(
            &mut self,
            request: impl tonic::IntoRequest<super::PrepareRocketLaunchSeqRequest>,
        ) -> Result<tonic::Response<super::PrepareRocketLaunchSeqResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/rockets.RocketLauncher/SendPreLaunch");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn send(
            &mut self,
            request: impl tonic::IntoRequest<super::RocketLaunchRequest>,
        ) -> Result<tonic::Response<super::RocketLaunchResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/rockets.RocketLauncher/Send");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn send_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::RocketLaunchRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::RocketLaunchResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/rockets.RocketLauncher/SendStream");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn receive_stream(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::RocketLaunchRequest>,
        ) -> Result<tonic::Response<super::RocketLaunchResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/rockets.RocketLauncher/ReceiveStream");
            self.inner
                .client_streaming(request.into_streaming_request(), path, codec)
                .await
        }
        pub async fn bidirectional(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::RocketLaunchRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::RocketLaunchResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/rockets.RocketLauncher/Bidirectional");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod rocket_launcher_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with RocketLauncherServer."]
    #[async_trait]
    pub trait RocketLauncher: Send + Sync + 'static {
        #[doc = " function which can be called"]
        #[doc = "Pre-launch API"]
        async fn send_pre_launch(
            &self,
            request: tonic::Request<super::PrepareRocketLaunchSeqRequest>,
        ) -> Result<tonic::Response<super::PrepareRocketLaunchSeqResponse>, tonic::Status>;
        async fn send(
            &self,
            request: tonic::Request<super::RocketLaunchRequest>,
        ) -> Result<tonic::Response<super::RocketLaunchResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the SendStream method."]
        type SendStreamStream: futures_core::Stream<Item = Result<super::RocketLaunchResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn send_stream(
            &self,
            request: tonic::Request<super::RocketLaunchRequest>,
        ) -> Result<tonic::Response<Self::SendStreamStream>, tonic::Status>;
        async fn receive_stream(
            &self,
            request: tonic::Request<tonic::Streaming<super::RocketLaunchRequest>>,
        ) -> Result<tonic::Response<super::RocketLaunchResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the Bidirectional method."]
        type BidirectionalStream: futures_core::Stream<Item = Result<super::RocketLaunchResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn bidirectional(
            &self,
            request: tonic::Request<tonic::Streaming<super::RocketLaunchRequest>>,
        ) -> Result<tonic::Response<Self::BidirectionalStream>, tonic::Status>;
    }
    #[doc = " service which can be executed"]
    #[derive(Debug)]
    pub struct RocketLauncherServer<T: RocketLauncher> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: RocketLauncher> RocketLauncherServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for RocketLauncherServer<T>
    where
        T: RocketLauncher,
        B: Body + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/rockets.RocketLauncher/SendPreLaunch" => {
                    #[allow(non_camel_case_types)]
                    struct SendPreLaunchSvc<T: RocketLauncher>(pub Arc<T>);
                    impl<T: RocketLauncher>
                        tonic::server::UnaryService<super::PrepareRocketLaunchSeqRequest>
                        for SendPreLaunchSvc<T>
                    {
                        type Response = super::PrepareRocketLaunchSeqResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PrepareRocketLaunchSeqRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).send_pre_launch(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendPreLaunchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rockets.RocketLauncher/Send" => {
                    #[allow(non_camel_case_types)]
                    struct SendSvc<T: RocketLauncher>(pub Arc<T>);
                    impl<T: RocketLauncher> tonic::server::UnaryService<super::RocketLaunchRequest> for SendSvc<T> {
                        type Response = super::RocketLaunchResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RocketLaunchRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).send(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rockets.RocketLauncher/SendStream" => {
                    #[allow(non_camel_case_types)]
                    struct SendStreamSvc<T: RocketLauncher>(pub Arc<T>);
                    impl<T: RocketLauncher>
                        tonic::server::ServerStreamingService<super::RocketLaunchRequest>
                        for SendStreamSvc<T>
                    {
                        type Response = super::RocketLaunchResponse;
                        type ResponseStream = T::SendStreamStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RocketLaunchRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).send_stream(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendStreamSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rockets.RocketLauncher/ReceiveStream" => {
                    #[allow(non_camel_case_types)]
                    struct ReceiveStreamSvc<T: RocketLauncher>(pub Arc<T>);
                    impl<T: RocketLauncher>
                        tonic::server::ClientStreamingService<super::RocketLaunchRequest>
                        for ReceiveStreamSvc<T>
                    {
                        type Response = super::RocketLaunchResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::RocketLaunchRequest>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).receive_stream(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReceiveStreamSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rockets.RocketLauncher/Bidirectional" => {
                    #[allow(non_camel_case_types)]
                    struct BidirectionalSvc<T: RocketLauncher>(pub Arc<T>);
                    impl<T: RocketLauncher>
                        tonic::server::StreamingService<super::RocketLaunchRequest>
                        for BidirectionalSvc<T>
                    {
                        type Response = super::RocketLaunchResponse;
                        type ResponseStream = T::BidirectionalStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::RocketLaunchRequest>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).bidirectional(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BidirectionalSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: RocketLauncher> Clone for RocketLauncherServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: RocketLauncher> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: RocketLauncher> tonic::transport::NamedService for RocketLauncherServer<T> {
        const NAME: &'static str = "rockets.RocketLauncher";
    }
}
