#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCustomerRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub email: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub phone: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomerRequest {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub email: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub phone: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomerRequest {
    #[prost(uint64, tag="1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomerResponse {
    #[prost(message, optional, tag="1")]
    pub customer: ::core::option::Option<Customer>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomerRequest {
    #[prost(string, optional, tag="1")]
    pub query: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag="2")]
    pub cursor: ::core::option::Option<u64>,
    #[prost(uint32, tag="3")]
    pub page_size: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomerResponse {
    #[prost(message, repeated, tag="1")]
    pub customers: ::prost::alloc::vec::Vec<Customer>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Customer {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub email: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub phone: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, tag="5")]
    pub created_at: u64,
    #[prost(uint64, optional, tag="6")]
    pub updated_at: ::core::option::Option<u64>,
}
/// Generated client implementations.
pub mod customer_services_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct CustomerServicesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CustomerServicesClient<tonic::transport::Channel> {
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
    impl<T> CustomerServicesClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Default + Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CustomerServicesClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
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
            CustomerServicesClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCustomerRequest>,
        ) -> Result<tonic::Response<super::Customer>, tonic::Status> {
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
                "/grpc.customer.CustomerServices/create",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCustomerRequest>,
        ) -> Result<tonic::Response<super::Customer>, tonic::Status> {
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
                "/grpc.customer.CustomerServices/update",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomerRequest>,
        ) -> Result<tonic::Response<super::GetCustomerResponse>, tonic::Status> {
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
                "/grpc.customer.CustomerServices/get",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCustomerRequest>,
        ) -> Result<tonic::Response<super::ListCustomerResponse>, tonic::Status> {
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
                "/grpc.customer.CustomerServices/list",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
