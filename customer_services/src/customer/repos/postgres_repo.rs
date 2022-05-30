/// references: https://qiita.com/FuJino/items/08b4c3298918191eab65
use std::ops::DerefMut;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use futures::FutureExt;
use futures::lock::Mutex;
use sea_query::{Cond, Query};
use sea_query::{Expr, PostgresQueryBuilder};
use sqlx::{Postgres, Row};
use sqlx::pool::PoolConnection;

use common::customer_pb::{CreateCustomerRequest, UpdateCustomerRequest};
use common::json::customer::{Customer, Customers};
use common::types::ListRequest;

use crate::customer::repos::repo::CustomerRepo;
use crate::ID_GENERATOR;

pub struct CustomerRepoImpl {
    session: Arc<Mutex<PoolConnection<Postgres>>>,
}

impl CustomerRepoImpl {
    pub(crate) fn new(session: Arc<Mutex<PoolConnection<Postgres>>>) -> Self {
        Self { session }
    }
}

#[async_trait]
impl CustomerRepo for CustomerRepoImpl {
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

        return Ok(sqlx::query_as::<_, Customer>(&sql)
            .fetch_optional(conn.deref_mut())
            .await?);
    }

    async fn create(&self, request: CreateCustomerRequest) -> Result<Customer> {
        use chrono::Utc;

        let mut conn = self.session.lock().await;
        let id = async move { ID_GENERATOR.clone().lock().unwrap().next_id() }
            .boxed()
            .await as u64;

        let name = request.name.clone().into();
        let email = request.email.into();
        let phone = request.phone.into();
        let created_at = Utc::now().into();

        let cols: Vec<Customers> = vec![
            Customers::Id,
            Customers::Name,
            Customers::Email,
            Customers::Phone,
            Customers::CreatedAt,
            Customers::UpdatedAt,
        ];

        let sql = Query::insert()
            .into_table(Customers::Table)
            .columns(cols.clone().into_iter().take(5).collect::<Vec<_>>())
            .values_panic(vec![id.into(), name, email, phone, created_at])
            .returning(Query::select().columns(cols).take())
            .to_string(PostgresQueryBuilder);

        return Ok(sqlx::query_as::<_, Customer>(&sql)
            .fetch_one(conn.deref_mut())
            .await?);
    }

    async fn list(&self, request: ListRequest) -> Result<Vec<Customer>> {
        let mut conn = self.session.lock().await;
        let query = request.query.map(|q| format!("%{}%", q));
        let page_size = request.page_size as u64;
        let offset = request.page as u64 * page_size;

        let sql = Query::select()
            .columns(vec![
                Customers::Id,
                Customers::Name,
                Customers::Email,
                Customers::Phone,
                Customers::CreatedAt,
                Customers::UpdatedAt,
            ])
            .cond_where(
                Cond::any()
                    .add_option(query.clone().map(|e| Expr::col(Customers::Name).like(&e)))
                    .add_option(query.clone().map(|e| Expr::col(Customers::Email).like(&e)))
                    .add_option(query.map(|e| Expr::col(Customers::Phone).like(&e))),
            )
            .from(Customers::Table)
            .offset(offset)
            .limit(page_size)
            .to_string(PostgresQueryBuilder);

        return Ok(sqlx::query_as::<_, Customer>(&sql)
            .fetch_all(&mut *conn)
            .await?);
    }

    async fn update(&self, request: UpdateCustomerRequest) -> Result<bool> {
        let mut conn = self.session.lock().await;
        let mut update_values = vec![];

        if let Some(name) = request.name {
            update_values.push((Customers::Name, name.into()));
        }

        if let Some(email) = request.email {
            update_values.push((Customers::Email, email.into()));
        }

        if let Some(phone) = request.phone {
            update_values.push((Customers::Phone, phone.into()));
        }

        let sql = Query::update()
            .table(Customers::Table)
            .values(update_values)
            .and_where(Expr::col(Customers::Id).eq(request.id))
            .to_string(PostgresQueryBuilder);

        return Ok(sqlx::query(&sql)
            .execute(conn.deref_mut())
            .await
            .map(|e| e.rows_affected() > 0)?);
    }

    async fn check_customer_is_exist(
        &self,
        phone: Option<String>,
        email: Option<String>,
    ) -> Result<bool> {
        let mut conn = self.session.lock().await;

        if phone.is_none() && email.is_none() {
            return Ok(false);
        }

        let sql = Query::select()
            .columns(vec![Customers::Id])
            .from(Customers::Table)
            .and_where_option(phone.map(|phone| Expr::col(Customers::Phone).eq(phone)))
            .and_where_option(email.map(|email| Expr::col(Customers::Email).eq(email)))
            .to_string(PostgresQueryBuilder);

        Ok(sqlx::query(&sql)
            .fetch_one(conn.deref_mut())
            .await
            .map(|row| row.len() > 1)?)
    }
}
