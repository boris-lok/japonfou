use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use tonic::{Request, Response, Status};

use common::order_item_pb::order_services_server::OrderServices;
use common::order_item_pb::{
    CreateOrderItemRequest, GetOrderItemResponse, ListOrderItemResponse, OrderItem,
    UpdateOrderItemRequest,
};
use common::types::{GetByIdRequest, ListRequest};
use common::util::tools::grpc_error_handler;

use crate::order::services::service::{OrderItemService, OrderItemServiceImpl};

pub struct GrpcOrderServiceImpl {
    session: Pool<Postgres>,
}

impl GrpcOrderServiceImpl {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { session: pool }
    }
}

#[async_trait]
impl OrderServices for GrpcOrderServiceImpl {
    async fn get(
        &self,
        request: Request<GetByIdRequest>,
    ) -> Result<Response<GetOrderItemResponse>, Status> {
        let id = request.into_inner().id;

        let services = OrderItemServiceImpl::new(self.session.clone());
        services
            .get(id)
            .await
            .map(|o| o.map(|e| e.into()))
            .map(|r| Response::new(GetOrderItemResponse { item: r }))
            .map_err(grpc_error_handler)
    }

    async fn list(
        &self,
        request: Request<ListRequest>,
    ) -> Result<Response<ListOrderItemResponse>, Status> {
        let req = request.into_inner();
        let services = OrderItemServiceImpl::new(self.session.clone());

        services
            .list(req)
            .await
            .map(|e| {
                let elements = e.into_iter().map(|o| o.into()).collect::<Vec<_>>();
                ListOrderItemResponse { items: elements }
            })
            .map(Response::new)
            .map_err(grpc_error_handler)
    }

    async fn update(
        &self,
        request: Request<UpdateOrderItemRequest>,
    ) -> Result<Response<OrderItem>, Status> {
        let req = request.into_inner();
        let services = OrderItemServiceImpl::new(self.session.clone());

        services
            .update(req)
            .await
            .map(|e| Response::new(e.into()))
            .map_err(grpc_error_handler)
    }

    async fn create(
        &self,
        request: Request<CreateOrderItemRequest>,
    ) -> Result<Response<OrderItem>, Status> {
        let services = OrderItemServiceImpl::new(self.session.clone());

        services
            .create(request.into_inner())
            .await
            .map(|o| Response::new(o.into()))
            .map_err(grpc_error_handler)
    }
}
