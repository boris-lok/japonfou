use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use common::json::customer::Customer;
use common::pb::{CreateCustomerRequest, ListCustomerRequest, UpdateCustomerRequest};
use common::util::alias::AppResult;
use common::util::errors::AppError;

use crate::customer::repos::postgres_repo::CustomerRepoImpl;
use crate::customer::repos::repo::CustomerRepo;

#[async_trait]
pub trait CustomerService {
    async fn get(&self, id: i64) -> AppResult<Option<Customer>>;

    async fn create(&self, request: CreateCustomerRequest) -> AppResult<Customer>;

    async fn list(&self, request: ListCustomerRequest) -> AppResult<Vec<Customer>>;

    async fn update(&self, request: UpdateCustomerRequest) -> AppResult<Customer>;
}

pub struct CustomerServiceImpl {
    pool: Pool<Postgres>,
}

impl CustomerServiceImpl {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CustomerService for CustomerServiceImpl {
    async fn get(&self, id: i64) -> AppResult<Option<Customer>> {
        let repo = CustomerRepoImpl;

        repo.get(id, &self.pool.clone()).await.map_err(|e| {
            let msg = e.to_string();
            AppError::DatabaseError(msg)
        })
    }

    async fn create(&self, request: CreateCustomerRequest) -> AppResult<Customer> {
        let repo = CustomerRepoImpl;

        let mut tx = self.pool.begin().await.unwrap();

        let customer = repo.create(request, &mut *tx).await.map_err(|e| {
            let msg = e.to_string();
            AppError::DatabaseError(msg)
        });

        let _ = tx.commit().await.unwrap();

        customer
    }

    async fn list(&self, request: ListCustomerRequest) -> AppResult<Vec<Customer>> {
        let repo = CustomerRepoImpl;

        repo.list(request, &self.pool.clone()).await.map_err(|e| {
            let msg = e.to_string();
            AppError::DatabaseError(msg)
        })
    }

    async fn update(&self, request: UpdateCustomerRequest) -> AppResult<Customer> {
        let repo = CustomerRepoImpl;

        let mut tx = self.pool.begin().await.unwrap();

        let old_customer = repo.get(request.id as i64, &mut *tx).await.ok().flatten();

        if let Some(c) = old_customer {
            let is_affected = repo.update(request.clone(), &mut *tx).await;

            tx.commit().await.unwrap();

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
            tx.rollback().await.unwrap();
        }

        Err(AppError::DatabaseError(
            "failed to update customer.".to_string(),
        ))
    }
}