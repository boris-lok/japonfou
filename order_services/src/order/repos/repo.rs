use anyhow::Result;
use async_trait::async_trait;

use common::json::order_item::OrderItem;
use common::order_item_pb::CreateOrderItemRequest;
use common::util::alias::PostgresAcquire;

#[async_trait]
pub trait OrderItemRepo {
    async fn get(
        &self,
        id: u64,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> Result<Option<OrderItem>>;

    async fn create(
        &self,
        req: CreateOrderItemRequest,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> Result<u64>;
}
