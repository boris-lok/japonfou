use std::ops::DerefMut;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use futures::lock::Mutex;
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sqlx::pool::PoolConnection;
use sqlx::Postgres;

use common::customer_pb::{CreateCustomerRequest, UpdateCustomerRequest};
use common::json::customer::{Customer, Customers};
use common::types::ListRequest;
use common::util::alias::PostgresAcquire;
use common::util::errors::AppError;

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

#[async_trait]
pub(crate) trait Repo {
    async fn get(&self, id: i64) -> Result<Option<Customer>>;
}

pub(crate) struct CRepoImpl {
    session: Arc<Mutex<PoolConnection<Postgres>>>,
}

impl CRepoImpl {
    pub fn new(session: Arc<Mutex<PoolConnection<Postgres>>>) -> Self {
        Self { session }
    }
}

#[async_trait]
impl Repo for CRepoImpl {
    async fn get(&self, id: i64) -> Result<Option<Customer>> {
        let mut conn = self.session.lock().await;
        let sql = Query::select()
            .columns(vec![
                Customers::Id,
                Customers::Name,
                Customers::Email,
                Customers::Phone,
                Customers::CreatedAt,
                Customers::UpdatedAt,
            ])
            .from(Customers::Table)
            .and_where(Expr::col(Customers::Id).eq(id))
            .to_string(PostgresQueryBuilder);

        let c = sqlx::query_as::<_, Customer>(&sql)
            .fetch_optional(conn.deref_mut())
            .await?;

        return Ok(None);
    }
}
