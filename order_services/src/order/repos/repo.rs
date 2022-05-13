use anyhow::Result;
use async_trait::async_trait;

use common::json::customer::Customer;
use common::json::order_item::OrderItem;
use common::json::product::Product;
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

#[async_trait]
pub trait ProductRepo {
    async fn get(
        &self,
        id: u64,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> Result<Option<Product>>;
}

#[async_trait]
pub trait CustomerRepo {
    async fn get(
        &self,
        id: u64,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> Result<Option<Customer>>;
}
