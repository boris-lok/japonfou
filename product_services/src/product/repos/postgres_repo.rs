use anyhow::Result;
use async_trait::async_trait;
use futures::lock::Mutex;
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sqlx::pool::PoolConnection;
use sqlx::Postgres;
use std::ops::DerefMut;
use std::sync::Arc;

use common::json::product::{Product, Products};
use common::product_pb::{CreateProductRequest, UpdateProductRequest};
use common::types::ListRequest;

use crate::product::repos::repo::ProductRepo;
use crate::ID_GENERATOR;

pub struct ProductRepoImpl {
    session: Arc<Mutex<PoolConnection<Postgres>>>,
}

impl ProductRepoImpl {
    pub(crate) fn new(session: Arc<Mutex<PoolConnection<Postgres>>>) -> Self {
        Self { session }
    }
}

#[async_trait]
impl ProductRepo for ProductRepoImpl {
    async fn get(&self, id: i64) -> Result<Option<Product>> {
        let mut conn = self.session.lock().await;

        let sql = Query::select()
            .columns([
                Products::Id,
                Products::Name,
                Products::Currency,
                Products::Price,
                Products::CreatedAt,
                Products::UpdatedAt,
                Products::DeletedAt,
            ])
            .from(Products::Table)
            .and_where(Expr::col(Products::Id).eq(id))
            .to_string(PostgresQueryBuilder);

        Ok(sqlx::query_as::<_, Product>(sql.as_str())
            .fetch_optional(conn.deref_mut())
            .await?)
    }

    async fn create(&self, request: CreateProductRequest) -> Result<Product> {
        let id = async move { ID_GENERATOR.lock().unwrap().next_id() as u64 }.await;

        let mut conn = self.session.lock().await;

        let name = request.name.clone().into();
        let currency = request.currency.into();
        let price = request.price.into();
        let now = chrono::Utc::now().into();

        let cols: Vec<Products> = vec![
            Products::Id,
            Products::Name,
            Products::Currency,
            Products::Price,
            Products::CreatedAt,
            Products::UpdatedAt,
            Products::DeletedAt,
        ];

        let sql = Query::insert()
            .into_table(Products::Table)
            .columns(cols.clone().into_iter().take(5).collect::<Vec<_>>())
            .values_panic(vec![id.into(), name, currency, price, now])
            .returning(Query::select().columns(cols).take())
            .to_string(PostgresQueryBuilder);

        Ok(sqlx::query_as::<_, Product>(&sql)
            .fetch_one(conn.deref_mut())
            .await?)
    }

    async fn update(&self, request: UpdateProductRequest) -> Result<bool> {
        let mut conn = self.session.lock().await;

        let mut update_values = vec![];
        if let Some(name) = request.name {
            update_values.push((Products::Name, name.into()));
        }

        if let Some(currency) = request.currency {
            update_values.push((Products::Currency, currency.into()));
        }

        if let Some(price) = request.price {
            update_values.push((Products::Price, price.into()));
        }

        if update_values.is_empty() {
            return Ok(false);
        }

        let sql = Query::update()
            .table(Products::Table)
            .values(update_values)
            .and_where(Expr::col(Products::Id).eq(request.id))
            .to_string(PostgresQueryBuilder);

        Ok(sqlx::query(&sql)
            .execute(conn.deref_mut())
            .await
            .map(|e| e.rows_affected() > 0)?)
    }

    async fn list(&self, request: ListRequest) -> Result<Vec<Product>> {
        let mut conn = self.session.lock().await;

        let query = request.query.map(|q| format!("%{}%", q));
        let page_size = request.page_size;
        let offset = request.page as u64 * page_size;

        let sql = Query::select()
            .columns(vec![
                Products::Id,
                Products::Name,
                Products::Currency,
                Products::Price,
                Products::CreatedAt,
                Products::UpdatedAt,
            ])
            .and_where_option(query.map(|e| Expr::col(Products::Name).like(&e)))
            .from(Products::Table)
            .offset(offset)
            .limit(page_size)
            .to_string(PostgresQueryBuilder);

        Ok(sqlx::query_as::<_, Product>(&sql)
            .fetch_all(conn.deref_mut())
            .await?)
    }
}
