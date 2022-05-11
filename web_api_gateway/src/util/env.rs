use tonic::transport::Channel;

use common::customer_pb::customer_services_client::CustomerServicesClient;
use common::order_item_pb::order_services_client::OrderServicesClient;
use common::product_pb::product_services_client::ProductServicesClient;

#[derive(Debug, Clone)]
pub struct Env {
    pub debug: bool,
    pub grpc_customer_client: CustomerServicesClient<Channel>,
    pub grpc_product_client: ProductServicesClient<Channel>,
    pub grpc_order_client: OrderServicesClient<Channel>,
}

impl Env {
    pub fn new(
        debug: bool,
        grpc_customer_client: CustomerServicesClient<Channel>,
        grpc_product_client: ProductServicesClient<Channel>,
        grpc_order_client: OrderServicesClient<Channel>,
    ) -> Self {
        Self {
            debug,
            grpc_customer_client,
            grpc_product_client,
            grpc_order_client,
        }
    }
}
