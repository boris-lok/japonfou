use async_trait::async_trait;
use futures::lock::Mutex;
use sqlx::pool::PoolConnection;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tonic::{Request, Response, Status};

use anyhow::Result;
use common::order_item_pb::order_services_server::OrderServices;
use common::order_item_pb::{
    CreateOrderItemRequest, GetOrderItemResponse, ListOrderItemResponse, OrderItem,
    UpdateOrderItemRequest, UpdateOrderItemsStatusRequest, UpdateOrderItemsStatusResponse,
};
use common::types::{GetByIdRequest, ListRequest};
use common::util::tools::grpc_error_handler;

use crate::order::services::service::{OrderItemService, OrderItemServiceImpl};

pub struct GrpcOrderServiceImpl {
    pool: Pool<Postgres>,
}

impl GrpcOrderServiceImpl {
    pub(crate) fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    async fn get_session(&self) -> Result<Arc<Mutex<PoolConnection<Postgres>>>> {
        let conn = self.pool.acquire().await?;

        Ok(Arc::new(Mutex::new(conn)))
    }
}

#[async_trait]
impl OrderServices for GrpcOrderServiceImpl {
    async fn get(
        &self,
        request: Request<GetByIdRequest>,
    ) -> Result<Response<GetOrderItemResponse>, Status> {
        let id = request.into_inner().id;
        let session = self.get_session().await.unwrap();

        let services = OrderItemServiceImpl::new(session);
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
        let session = self.get_session().await.unwrap();

        let services = OrderItemServiceImpl::new(session);
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
        let session = self.get_session().await.unwrap();

        let services = OrderItemServiceImpl::new(session);
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
        let session = self.get_session().await.unwrap();

        let services = OrderItemServiceImpl::new(session);
        services
            .create(request.into_inner())
            .await
            .map(|o| Response::new(o.into()))
            .map_err(grpc_error_handler)
    }

    async fn update_order_items_status(
        &self,
        request: Request<UpdateOrderItemsStatusRequest>,
    ) -> Result<Response<UpdateOrderItemsStatusResponse>, Status> {
        let session = self.get_session().await.unwrap();

        let services = OrderItemServiceImpl::new(session);
        services
            .update_items_status(request.into_inner())
            .await
            .map(|e| UpdateOrderItemsStatusResponse { result: e })
            .map(Response::new)
            .map_err(grpc_error_handler)
    }
}
