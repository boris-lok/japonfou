use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use common::json::order_item::OrderItem;
use common::order_item_pb::CreateOrderItemRequest;
use common::util::alias::AppResult;
use common::util::errors::AppError;

use crate::order::repos::postgres_repo::OrderItemRepoImpl;
use crate::order::repos::repo::OrderItemRepo;

#[async_trait]
pub trait OrderItemService {
    async fn get(&self, id: u64) -> AppResult<Option<OrderItem>>;

    async fn create(self, req: CreateOrderItemRequest) -> AppResult<OrderItem>;
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
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    async fn create(self, req: CreateOrderItemRequest) -> AppResult<OrderItem> {
        let repo = OrderItemRepoImpl;

        let mut tx = self.pool.begin().await.unwrap();

        let result = repo
            .create(req, &mut *tx)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()));

        if let Ok(new_id) = result {
            let item = repo
                .get(new_id, &mut *tx)
                .await
                .map(|o| o.unwrap())
                .map_err(|err| AppError::DatabaseError(err.to_string()));

            tx.commit().await.unwrap();

            return item;
        } else {
            tx.rollback().await.unwrap();
        }

        Err(AppError::DatabaseError(result.err().unwrap().to_string()))
    }
}
