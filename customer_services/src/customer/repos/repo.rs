use anyhow::Result;
use async_trait::async_trait;

use common::customer_pb::{CreateCustomerRequest, UpdateCustomerRequest};
use common::json::customer::Customer;
use common::types::ListRequest;

#[async_trait]
pub(crate) trait CustomerRepo {
    async fn get(&self, id: i64) -> Result<Option<Customer>>;
    async fn create(&self, req: CreateCustomerRequest) -> Result<Customer>;
    async fn list(&self, req: ListRequest) -> Result<Vec<Customer>>;
    async fn update(&self, req: UpdateCustomerRequest) -> Result<bool>;
}
