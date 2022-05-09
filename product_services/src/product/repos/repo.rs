use anyhow::Result;
use async_trait::async_trait;

use common::json::product::Product;
use common::pb::{CreateProductRequest, ListProductRequest, UpdateProductRequest};
use common::util::alias::PostgresAcquire;

#[async_trait]
pub trait ProductRepo {
    async fn get(
        &self,
        id: i64,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> Result<Option<Product>>;

    async fn create(
        &self,
        request: CreateProductRequest,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> Result<Product>;

    async fn update(
        &self,
        request: UpdateProductRequest,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> Result<bool>;

    async fn list(
        &self,
        request: ListProductRequest,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> Result<Vec<Product>>;
}
