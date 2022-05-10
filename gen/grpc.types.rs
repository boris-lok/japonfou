#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetByIdRequest {
    #[prost(uint64, tag="1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRequest {
    #[prost(string, optional, tag="1")]
    pub query: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, tag="2")]
    pub page: u64,
    #[prost(uint64, tag="3")]
    pub page_size: u64,
}
