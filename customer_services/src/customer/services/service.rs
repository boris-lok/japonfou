use anyhow::Result;
use async_trait::async_trait;
use futures::lock::Mutex;
use sqlx::pool::PoolConnection;
use sqlx::Postgres;
use std::ops::DerefMut;
use std::sync::Arc;

use common::customer_pb::{CreateCustomerRequest, UpdateCustomerRequest};
use common::json::customer::Customer;
use common::types::ListRequest;
use common::util::alias::AppResult;
use common::util::errors::AppError;
use common::util::tools::database_error_handler;

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

    async fn begin(&self) -> Result<bool> {
        let mut conn = self.session.lock().await;

        Ok(sqlx::query("BEGIN;")
            .execute(conn.deref_mut())
            .await
            .map(|row| row.rows_affected() > 0)?)
    }

    async fn commit(&self) -> Result<bool> {
        let mut conn = self.session.lock().await;

        Ok(sqlx::query("COMMIT;")
            .execute(conn.deref_mut())
            .await
            .map(|row| row.rows_affected() > 0)?)
    }

    async fn rollback(&self) -> Result<bool> {
        let mut conn = self.session.lock().await;

        Ok(sqlx::query("ROLLBACK;")
            .execute(conn.deref_mut())
            .await
            .map(|row| row.rows_affected() > 0)?)
    }
}

#[async_trait]
impl CustomerService for CustomerServiceImpl {
    async fn get(&self, id: i64) -> AppResult<Option<Customer>> {
        self.repo.get(id).await.map_err(database_error_handler)
    }

    async fn create(&self, request: CreateCustomerRequest) -> AppResult<Customer> {
        // TODO: check and validate the data.
        self.repo.create(request).await.map_err(database_error_handler)
    }

    async fn list(&self, request: ListRequest) -> AppResult<Vec<Customer>> {
        self.repo.list(request).await.map_err(database_error_handler)
    }

    async fn update(&self, request: UpdateCustomerRequest) -> AppResult<Customer> {
        let _ = self.begin().await;

        let old_customer = self.repo.get(request.id as i64).await.ok().flatten();

        if let Some(c) = old_customer {
            let is_affected = self.repo.update(request.clone()).await;

            let _ = self.commit().await;

            if is_affected.is_ok() {
                let new_customer = Customer {
                    name: request.name.unwrap_or(c.name),
                    email: request.email.to_owned(),
                    phone: request.phone.to_owned(),
                    ..c
                };

                return Ok(new_customer);
            }

            return Ok(c);
        } else {
            let _ = self.rollback().await;
        }

        Err(AppError::DatabaseError(
            "failed to update customer.".to_string(),
        ))
    }
}
