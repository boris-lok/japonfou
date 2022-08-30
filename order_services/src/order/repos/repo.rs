use anyhow::Result;
use async_trait::async_trait;

use common::json::customer::Customer;
use common::json::order_item::OrderItem;
use common::json::product::Product;
use common::order_item_pb::{
    CreateOrderItemRequest, UpdateOrderItemRequest, UpdateOrderItemsStatusRequest,
};
use common::types::ListRequest;

#[async_trait]
pub trait OrderItemRepo {
    async fn get(&self, id: u64) -> Result<Option<OrderItem>>;

    async fn create(&self, req: CreateOrderItemRequest) -> Result<u64>;

    async fn list(&self, req: ListRequest) -> Result<Vec<OrderItem>>;

    async fn update(&self, req: UpdateOrderItemRequest) -> Result<bool>;

    async fn update_items_status(&self, req: UpdateOrderItemsStatusRequest) -> Result<bool>;
}

#[async_trait]
pub trait ProductRepo {
    async fn get(&self, id: u64) -> Result<Option<Product>>;
}

#[async_trait]
pub trait CustomerRepo {
    async fn get(&self, id: u64) -> Result<Option<Customer>>;
}
