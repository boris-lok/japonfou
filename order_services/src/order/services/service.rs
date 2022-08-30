use async_trait::async_trait;
use futures::lock::Mutex;
use sqlx::pool::PoolConnection;
use sqlx::Postgres;
use std::sync::Arc;

use common::json::order_item::OrderItem;
use common::order_item_pb::{
    CreateOrderItemRequest, UpdateOrderItemRequest, UpdateOrderItemsStatusRequest,
};
use common::types::ListRequest;
use common::util::alias::AppResult;
use common::util::errors::AppError;
use common::util::tools::database_error_handler;

use crate::order::repos::postgres_repo::{CustomerRepoImpl, OrderItemRepoImpl, ProductRepoImpl};
use crate::order::repos::repo::{CustomerRepo, OrderItemRepo, ProductRepo};

#[async_trait]
pub trait OrderItemService {
    async fn get(&self, id: u64) -> AppResult<Option<OrderItem>>;

    async fn create(self, req: CreateOrderItemRequest) -> AppResult<OrderItem>;

    async fn list(self, req: ListRequest) -> AppResult<Vec<OrderItem>>;

    async fn update(self, req: UpdateOrderItemRequest) -> AppResult<OrderItem>;

    async fn update_items_status(self, req: UpdateOrderItemsStatusRequest) -> AppResult<bool>;
}

pub struct OrderItemServiceImpl {
    // session: Arc<Mutex<PoolConnection<Postgres>>>,
    order_repo: Box<dyn OrderItemRepo + Send + Sync>,
    product_repo: Box<dyn ProductRepo + Send + Sync>,
    customer_repo: Box<dyn CustomerRepo + Send + Sync>,
}

impl OrderItemServiceImpl {
    pub(crate) fn new(session: Arc<Mutex<PoolConnection<Postgres>>>) -> Self {
        let order_repo = Box::new(OrderItemRepoImpl::new(session.clone()));
        let product_repo = Box::new(ProductRepoImpl::new(session.clone()));
        let customer_repo = Box::new(CustomerRepoImpl::new(session.clone()));

        Self {
            // session,
            order_repo,
            product_repo,
            customer_repo,
        }
    }
}

#[async_trait]
impl OrderItemService for OrderItemServiceImpl {
    async fn get(&self, id: u64) -> AppResult<Option<OrderItem>> {
        self.order_repo
            .get(id)
            .await
            .map_err(database_error_handler)
    }

    async fn create(self, req: CreateOrderItemRequest) -> AppResult<OrderItem> {
        let product = self.product_repo.get(req.product_id).await;

        if product.is_err() {
            return Err(AppError::DatabaseError(product.err().unwrap().to_string()));
        }

        if product.unwrap().is_none() {
            let msg = format!(
                "Product {} doesn't exist, when create a order.",
                req.product_id
            );
            return Err(AppError::BadRequest(msg));
        }

        let customer = self.customer_repo.get(req.customer_id).await;

        if customer.is_err() {
            return Err(AppError::DatabaseError(customer.err().unwrap().to_string()));
        }

        if customer.unwrap().is_none() {
            let msg = format!(
                "Customer {} doesn't exist, when create a order",
                req.customer_id
            );
            return Err(AppError::BadRequest(msg));
        }

        let result = self
            .order_repo
            .create(req)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()));

        if let Ok(new_id) = result {
            let item = self
                .order_repo
                .get(new_id)
                .await
                .map(|o| o.unwrap())
                .map_err(database_error_handler);

            return item;
        }

        Err(AppError::DatabaseError(result.err().unwrap().to_string()))
    }

    async fn list(self, req: ListRequest) -> AppResult<Vec<OrderItem>> {
        self.order_repo
            .list(req)
            .await
            .map_err(database_error_handler)
    }

    async fn update(self, req: UpdateOrderItemRequest) -> AppResult<OrderItem> {
        let old_order_item = self.order_repo.get(req.id).await.ok().flatten();

        if old_order_item.is_none() {
            return Err(AppError::BadRequest(format!(
                "Can't find the order item by id: {}",
                req.id
            )));
        }

        if let Some(customer_id) = req.customer_id {
            let customer = self.customer_repo.get(customer_id).await.ok().flatten();

            if customer.is_none() {
                return Err(AppError::BadRequest(format!(
                    "Can't update the order item by id: {}, because customer {} is not exist.",
                    req.id, customer_id
                )));
            }
        }

        if let Some(product_id) = req.product_id {
            let product = self.product_repo.get(product_id).await.ok().flatten();

            if product.is_none() {
                return Err(AppError::BadRequest(format!(
                    "Can't update the order item by id: {}, because product {} is not exist.",
                    req.id, product_id
                )));
            }
        }

        let id = req.id;
        let is_affected = self.order_repo.update(req).await;

        if is_affected.is_ok() {
            let new_order_item = self
                .order_repo
                .get(id)
                .await
                .map(|o| o.unwrap())
                .map_err(database_error_handler);
            return new_order_item;
        }

        return Err(AppError::DatabaseError(format!(
            "Can't update order item by id: {}",
            id
        )));
    }

    async fn update_items_status(self, req: UpdateOrderItemsStatusRequest) -> AppResult<bool> {
        self.order_repo
            .update_items_status(req)
            .await
            .map_err(database_error_handler)
    }
}
