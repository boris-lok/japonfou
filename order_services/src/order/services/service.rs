use async_trait::async_trait;
use sqlx::{Pool, Postgres};

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
    pool: Pool<Postgres>,
}

impl OrderItemServiceImpl {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OrderItemService for OrderItemServiceImpl {
    async fn get(&self, id: u64) -> AppResult<Option<OrderItem>> {
        let repo = OrderItemRepoImpl;

        repo.get(id, &self.pool.clone())
            .await
            .map_err(database_error_handler)
    }

    async fn create(self, req: CreateOrderItemRequest) -> AppResult<OrderItem> {
        let order_item_repo = OrderItemRepoImpl;
        let product_repo = ProductRepoImpl;
        let customer_repo = CustomerRepoImpl;

        let mut tx = self.pool.begin().await.unwrap();

        let product = product_repo.get(req.product_id, &mut *tx).await;

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

        let customer = customer_repo.get(req.customer_id, &mut *tx).await;

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

        let result = order_item_repo
            .create(req, &mut *tx)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()));

        if let Ok(new_id) = result {
            let item = order_item_repo
                .get(new_id, &mut *tx)
                .await
                .map(|o| o.unwrap())
                .map_err(database_error_handler);

            tx.commit().await.unwrap();

            return item;
        } else {
            tx.rollback().await.unwrap();
        }

        Err(AppError::DatabaseError(result.err().unwrap().to_string()))
    }

    async fn list(self, req: ListRequest) -> AppResult<Vec<OrderItem>> {
        let repo = OrderItemRepoImpl;

        repo.list(req, &self.pool.clone())
            .await
            .map_err(database_error_handler)
    }

    async fn update(self, req: UpdateOrderItemRequest) -> AppResult<OrderItem> {
        let order_item_repo = OrderItemRepoImpl;
        let customer_repo = CustomerRepoImpl;
        let product_repo = ProductRepoImpl;

        let mut tx = self.pool.begin().await.unwrap();

        let old_order_item = order_item_repo.get(req.id, &mut *tx).await.ok().flatten();

        if old_order_item.is_none() {
            tx.rollback().await.unwrap();
            return Err(AppError::BadRequest(format!(
                "Can't find the order item by id: {}",
                req.id
            )));
        }

        if let Some(customer_id) = req.customer_id {
            let customer = customer_repo
                .get(customer_id, &mut *tx)
                .await
                .ok()
                .flatten();

            if customer.is_none() {
                tx.rollback().await.unwrap();
                return Err(AppError::BadRequest(format!(
                    "Can't update the order item by id: {}, because customer {} is not exist.",
                    req.id, customer_id
                )));
            }
        }

        if let Some(product_id) = req.product_id {
            let product = product_repo.get(product_id, &mut *tx).await.ok().flatten();

            if product.is_none() {
                tx.rollback().await.unwrap();
                return Err(AppError::BadRequest(format!(
                    "Can't update the order item by id: {}, because product {} is not exist.",
                    req.id, product_id
                )));
            }
        }

        let id = req.id;
        let is_affected = order_item_repo.update(req, &mut *tx).await;

        tx.commit().await.unwrap();

        if is_affected.is_ok() {
            let new_order_item = order_item_repo
                .get(id, &self.pool.clone())
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
        let repo = OrderItemRepoImpl;

        repo.update_items_status(req, &self.pool.clone())
            .await
            .map_err(database_error_handler)
    }
}
