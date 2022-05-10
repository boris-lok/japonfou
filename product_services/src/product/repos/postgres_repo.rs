use anyhow::Result;
use async_trait::async_trait;
use sea_query::{Expr, PostgresQueryBuilder, Query};

use common::json::product::{Product, Products};
use common::pb::{CreateProductRequest, UpdateProductRequest};
use common::types::ListRequest;
use common::util::alias::PostgresAcquire;

use crate::product::repos::repo::ProductRepo;
use crate::ID_GENERATOR;

pub struct ProductRepoImpl;

#[async_trait]
impl ProductRepo for ProductRepoImpl {
    async fn get(
        &self,
        id: i64,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> Result<Option<Product>> {
        let mut conn = executor.acquire().await.unwrap();

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
            .fetch_optional(&mut *conn)
            .await?)
    }

    async fn create(
        &self,
        request: CreateProductRequest,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> Result<Product> {
        let id = async move { ID_GENERATOR.lock().unwrap().next_id() as u64 }.await;

        let mut conn = executor.acquire().await.unwrap();

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
        ];

        let sql = Query::insert()
            .into_table(Products::Table)
            .columns(cols.clone())
            .values_panic(vec![id.into(), name, currency, price, now])
            .returning(Query::select().columns(cols).take())
            .to_string(PostgresQueryBuilder);

        Ok(sqlx::query_as::<_, Product>(&sql)
            .fetch_one(&mut *conn)
            .await?)
    }

    async fn update(
        &self,
        request: UpdateProductRequest,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> Result<bool> {
        let mut conn = executor.acquire().await.unwrap();

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

        let sql = Query::update()
            .table(Products::Table)
            .values(update_values)
            .and_where(Expr::col(Products::Id).eq(request.id))
            .to_string(PostgresQueryBuilder);

        Ok(sqlx::query(&sql)
            .execute(&mut *conn)
            .await
            .map(|e| e.rows_affected() > 0)?)
    }

    async fn list(
        &self,
        request: ListRequest,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> Result<Vec<Product>> {
        let mut conn = executor.acquire().await.unwrap();

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
            .fetch_all(&mut *conn)
            .await?)
    }
}
