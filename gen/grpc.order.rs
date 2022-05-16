#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrderItemResponse {
    #[prost(message, optional, tag="1")]
    pub item: ::core::option::Option<OrderItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrderItemResponse {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<OrderItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOrderItemRequest {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(uint64, optional, tag="2")]
    pub customer_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub product_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="4")]
    pub quantity: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub status: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOrderItemRequest {
    #[prost(uint64, tag="1")]
    pub customer_id: u64,
    #[prost(uint64, tag="2")]
    pub product_id: u64,
    #[prost(uint32, tag="3")]
    pub quantity: u32,
    #[prost(uint32, tag="4")]
    pub status: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderItem {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(message, optional, tag="2")]
    pub product: ::core::option::Option<order_item::Product>,
    #[prost(message, optional, tag="3")]
    pub customer: ::core::option::Option<order_item::Customer>,
    #[prost(uint32, tag="4")]
    pub quantity: u32,
    #[prost(uint64, tag="5")]
    pub created_at: u64,
    #[prost(uint64, optional, tag="6")]
    pub updated_at: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="7")]
    pub deleted_at: ::core::option::Option<u64>,
    #[prost(uint32, tag="8")]
    pub status: u32,
}
/// Nested message and enum types in `OrderItem`.
pub mod order_item {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Customer {
        #[prost(uint64, tag="1")]
        pub id: u64,
        #[prost(string, tag="2")]
        pub name: ::prost::alloc::string::String,
        #[prost(uint64, tag="3")]
        pub created_at: u64,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Product {
        #[prost(uint64, tag="1")]
        pub id: u64,
        #[prost(string, tag="2")]
        pub name: ::prost::alloc::string::String,
        #[prost(uint32, tag="3")]
        pub currency: u32,
        #[prost(double, tag="4")]
        pub price: f64,
        #[prost(uint64, tag="5")]
        pub created_at: u64,
    }
}
/// Generated client implementations.
pub mod order_services_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct OrderServicesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OrderServicesClient<tonic::transport::Channel> {
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
    impl<T> OrderServicesClient<T>
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> OrderServicesClient<InterceptedService<T, F>>
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
            OrderServicesClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::super::types::GetByIdRequest>,
        ) -> Result<tonic::Response<super::GetOrderItemResponse>, tonic::Status> {
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
                "/grpc.order.OrderServices/get",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::super::types::ListRequest>,
        ) -> Result<tonic::Response<super::ListOrderItemResponse>, tonic::Status> {
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
                "/grpc.order.OrderServices/list",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateOrderItemRequest>,
        ) -> Result<tonic::Response<super::OrderItem>, tonic::Status> {
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
                "/grpc.order.OrderServices/update",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateOrderItemRequest>,
        ) -> Result<tonic::Response<super::OrderItem>, tonic::Status> {
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
                "/grpc.order.OrderServices/create",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod order_services_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with OrderServicesServer.
    #[async_trait]
    pub trait OrderServices: Send + Sync + 'static {
        async fn get(
            &self,
            request: tonic::Request<super::super::types::GetByIdRequest>,
        ) -> Result<tonic::Response<super::GetOrderItemResponse>, tonic::Status>;
        async fn list(
            &self,
            request: tonic::Request<super::super::types::ListRequest>,
        ) -> Result<tonic::Response<super::ListOrderItemResponse>, tonic::Status>;
        async fn update(
            &self,
            request: tonic::Request<super::UpdateOrderItemRequest>,
        ) -> Result<tonic::Response<super::OrderItem>, tonic::Status>;
        async fn create(
            &self,
            request: tonic::Request<super::CreateOrderItemRequest>,
        ) -> Result<tonic::Response<super::OrderItem>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct OrderServicesServer<T: OrderServices> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: OrderServices> OrderServicesServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for OrderServicesServer<T>
    where
        T: OrderServices,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/grpc.order.OrderServices/get" => {
                    #[allow(non_camel_case_types)]
                    struct getSvc<T: OrderServices>(pub Arc<T>);
                    impl<
                        T: OrderServices,
                    > tonic::server::UnaryService<super::super::types::GetByIdRequest>
                    for getSvc<T> {
                        type Response = super::GetOrderItemResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::types::GetByIdRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = getSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grpc.order.OrderServices/list" => {
                    #[allow(non_camel_case_types)]
                    struct listSvc<T: OrderServices>(pub Arc<T>);
                    impl<
                        T: OrderServices,
                    > tonic::server::UnaryService<super::super::types::ListRequest>
                    for listSvc<T> {
                        type Response = super::ListOrderItemResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::types::ListRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = listSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grpc.order.OrderServices/update" => {
                    #[allow(non_camel_case_types)]
                    struct updateSvc<T: OrderServices>(pub Arc<T>);
                    impl<
                        T: OrderServices,
                    > tonic::server::UnaryService<super::UpdateOrderItemRequest>
                    for updateSvc<T> {
                        type Response = super::OrderItem;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateOrderItemRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = updateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grpc.order.OrderServices/create" => {
                    #[allow(non_camel_case_types)]
                    struct createSvc<T: OrderServices>(pub Arc<T>);
                    impl<
                        T: OrderServices,
                    > tonic::server::UnaryService<super::CreateOrderItemRequest>
                    for createSvc<T> {
                        type Response = super::OrderItem;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateOrderItemRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = createSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: OrderServices> Clone for OrderServicesServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: OrderServices> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: OrderServices> tonic::transport::NamedService for OrderServicesServer<T> {
        const NAME: &'static str = "grpc.order.OrderServices";
    }
}
