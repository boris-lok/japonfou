use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::Mutex;
use sqlx::pool::PoolConnection;
use sqlx::Postgres;

use common::customer_pb::{CreateCustomerRequest, UpdateCustomerRequest};
use common::json::customer::Customer;
use common::types::ListRequest;
use common::util::alias::AppResult;
use common::util::errors::AppError;
use common::util::tools::{
    begin_transaction, commit_transaction, database_error_handler, rollback_transaction,
};

use crate::customer::repos::postgres_repo::CustomerRepoImpl;
use crate::customer::repos::repo::CustomerRepo;

#[async_trait]
pub(crate) trait CustomerService {
    async fn get(&self, id: i64) -> AppResult<Option<Customer>>;

    async fn create(&self, request: CreateCustomerRequest) -> AppResult<Customer>;

    async fn list(&self, request: ListRequest) -> AppResult<Vec<Customer>>;

    async fn update(&self, request: UpdateCustomerRequest) -> AppResult<Customer>;
}

pub(crate) struct CustomerServiceImpl {
    session: Arc<Mutex<PoolConnection<Postgres>>>,
    repo: Box<dyn CustomerRepo + Sync + Send>,
}

impl CustomerServiceImpl {
    pub fn new(session: Arc<Mutex<PoolConnection<Postgres>>>) -> Self {
        let repo = CustomerRepoImpl::new(session.clone());
        Self {
            session,
            repo: Box::new(repo),
        }
    }
}

#[async_trait]
impl CustomerService for CustomerServiceImpl {
    async fn get(&self, id: i64) -> AppResult<Option<Customer>> {
        self.repo.get(id).await.map_err(database_error_handler)
    }

    async fn create(&self, request: CreateCustomerRequest) -> AppResult<Customer> {
        let is_exist = self
            .repo
            .check_customer_is_exist(request.phone.clone(), request.email.clone())
            .await;

        if is_exist.is_err() {
            return Err(AppError::DatabaseError(
                "Can't check the customer is exist or not.".to_string(),
            ));
        }

        if is_exist.is_ok() && is_exist.unwrap() {
            return Err(AppError::BadRequest("customer already exist.".to_string()));
        }

        self.repo
            .create(request)
            .await
            .map_err(database_error_handler)
    }

    async fn list(&self, request: ListRequest) -> AppResult<Vec<Customer>> {
        self.repo
            .list(request)
            .await
            .map_err(database_error_handler)
    }

    async fn update(&self, request: UpdateCustomerRequest) -> AppResult<Customer> {
        let _ = begin_transaction(self.session.clone()).await;
        let old_customer = self.repo.get(request.id as i64).await.ok().flatten();

        if old_customer.is_none() {
            let _ = rollback_transaction(self.session.clone()).await;
            return Err(AppError::BadRequest(format!(
                "Can't find the customer by id {}",
                request.id
            )));
        }

        let old_customer = old_customer.unwrap();

        let is_affected = self.repo.update(request.clone()).await;

        let _ = commit_transaction(self.session.clone()).await;

        if is_affected.is_ok() {
            let new_customer = Customer {
                name: request.name.unwrap_or(old_customer.name),
                email: request.email.to_owned(),
                phone: request.phone.to_owned(),
                ..old_customer
            };

            return Ok(new_customer);
        }

        return Ok(old_customer);
    }
}
