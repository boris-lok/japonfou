use tonic::transport::Channel;

use common::pb::customer_services_client::CustomerServicesClient;
use common::pb::product_services_client::ProductServicesClient;

#[derive(Debug, Clone)]
pub struct Env {
    pub debug: bool,
    pub grpc_customer_client: CustomerServicesClient<Channel>,
    pub grpc_product_client: ProductServicesClient<Channel>,
}

impl Env {
    pub fn new(
        debug: bool,
        grpc_customer_client: CustomerServicesClient<Channel>,
        grpc_product_client: ProductServicesClient<Channel>,
    ) -> Self {
        Self {
            debug,
            grpc_customer_client,
            grpc_product_client,
        }
    }
}
