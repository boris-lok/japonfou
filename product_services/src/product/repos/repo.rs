use anyhow::Result;
use async_trait::async_trait;

use common::json::product::Product;
use common::product_pb::{CreateProductRequest, UpdateProductRequest};
use common::types::ListRequest;

#[async_trait]
pub trait ProductRepo {
    async fn get(&self, id: i64) -> Result<Option<Product>>;

    async fn create(&self, request: CreateProductRequest) -> Result<Product>;

    async fn update(&self, request: UpdateProductRequest) -> Result<bool>;

    async fn list(&self, request: ListRequest) -> Result<Vec<Product>>;
}
