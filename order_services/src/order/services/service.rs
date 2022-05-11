use crate::order::repos::postgres_repo::OrderItemRepoImpl;
use crate::order::repos::repo::OrderItemRepo;
use async_trait::async_trait;
use common::json::order_item::OrderItem;
use common::util::alias::AppResult;
use common::util::errors::AppError;
use sqlx::{Pool, Postgres};

#[async_trait]
pub trait OrderItemService {
    async fn get(&self, id: u64) -> AppResult<Option<OrderItem>>;
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
}
