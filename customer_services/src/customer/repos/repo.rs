use anyhow::Result;
use async_trait::async_trait;

use common::customer_pb::{CreateCustomerRequest, UpdateCustomerRequest};
use common::json::customer::Customer;
use common::types::ListRequest;
use common::util::alias::PostgresAcquire;

#[async_trait]
pub trait CustomerRepo {
    async fn get(
        &self,
        id: i64,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> Result<Option<Customer>>;

    async fn create(
        &self,
        request: CreateCustomerRequest,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> Result<Customer>;

    async fn list(
        &self,
        request: ListRequest,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> Result<Vec<Customer>>;

    async fn update(
        &self,
        request: UpdateCustomerRequest,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> Result<bool>;
}
