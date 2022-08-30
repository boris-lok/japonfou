use async_trait::async_trait;
use futures::lock::Mutex;
use futures::FutureExt;
use sea_query::{Cond, Expr, JoinType, PostgresQueryBuilder, Query};
use sqlx::pool::PoolConnection;
use sqlx::Postgres;
use std::ops::DerefMut;
use std::sync::Arc;

use common::json::customer::{Customer, Customers};
use common::json::order_item::{OrderItem, OrderItems};
use common::json::product::{Product, Products};
use common::order_item_pb::{
    CreateOrderItemRequest, UpdateOrderItemRequest, UpdateOrderItemsStatusRequest,
};
use common::types::ListRequest;

use crate::order::repos::repo::{CustomerRepo, OrderItemRepo, ProductRepo};
use crate::ID_GENERATOR;

pub(crate) struct OrderItemRepoImpl {
    session: Arc<Mutex<PoolConnection<Postgres>>>,
}

impl OrderItemRepoImpl {
    pub(crate) fn new(session: Arc<Mutex<PoolConnection<Postgres>>>) -> Self {
        Self { session }
    }
}

pub(crate) struct ProductRepoImpl {
    session: Arc<Mutex<PoolConnection<Postgres>>>,
}

impl ProductRepoImpl {
    pub(crate) fn new(session: Arc<Mutex<PoolConnection<Postgres>>>) -> Self {
        Self { session }
    }
}

pub(crate) struct CustomerRepoImpl {
    session: Arc<Mutex<PoolConnection<Postgres>>>,
}

impl CustomerRepoImpl {
    pub(crate) fn new(session: Arc<Mutex<PoolConnection<Postgres>>>) -> Self {
        Self { session }
    }
}

#[async_trait]
impl OrderItemRepo for OrderItemRepoImpl {
    async fn get(&self, id: u64) -> anyhow::Result<Option<OrderItem>> {
        let mut conn = self.session.lock().await;

        let order_item_cols = vec![
            (OrderItems::Table, OrderItems::Id),
            (OrderItems::Table, OrderItems::Quantity),
            (OrderItems::Table, OrderItems::Status),
            (OrderItems::Table, OrderItems::CreatedAt),
            (OrderItems::Table, OrderItems::UpdatedAt),
            (OrderItems::Table, OrderItems::DeletedAt),
            (OrderItems::Table, OrderItems::CustomerId),
            (OrderItems::Table, OrderItems::ProductId),
        ];
        let customer_cols = vec![
            (Customers::Table, Customers::Name),
            (Customers::Table, Customers::CreatedAt),
        ];
        let product_cols = vec![
            (Products::Table, Products::Name),
            (Products::Table, Products::Currency),
            (Products::Table, Products::Price),
            (Products::Table, Products::CreatedAt),
        ];

        let sql = Query::select()
            .columns(order_item_cols)
            .columns(customer_cols)
            .columns(product_cols)
            .from(OrderItems::Table)
            .join(
                JoinType::InnerJoin,
                Customers::Table,
                Expr::tbl(OrderItems::Table, OrderItems::CustomerId)
                    .equals(Customers::Table, Customers::Id),
            )
            .join(
                JoinType::InnerJoin,
                Products::Table,
                Expr::tbl(OrderItems::Table, OrderItems::ProductId)
                    .equals(Products::Table, Products::Id),
            )
            .and_where(Expr::tbl(OrderItems::Table, OrderItems::Id).eq(id))
            .to_string(PostgresQueryBuilder);

        Ok(sqlx::query_as::<_, OrderItem>(&sql)
            .fetch_optional(conn.deref_mut())
            .await?)
    }

    async fn create(&self, req: CreateOrderItemRequest) -> anyhow::Result<u64> {
        let mut conn = self.session.lock().await;

        let id = async move { ID_GENERATOR.clone().lock().unwrap().next_id() }
            .boxed()
            .await as u64;

        let sql = Query::insert()
            .into_table(OrderItems::Table)
            .columns(vec![
                OrderItems::Id,
                OrderItems::CustomerId,
                OrderItems::ProductId,
                OrderItems::Quantity,
                OrderItems::Status,
                OrderItems::CreatedAt,
            ])
            .values_panic(vec![
                id.into(),
                req.customer_id.into(),
                req.product_id.into(),
                req.quantity.into(),
                req.status.into(),
                chrono::Utc::now().into(),
            ])
            .to_string(PostgresQueryBuilder);

        let _ = sqlx::query(&sql).execute(conn.deref_mut()).await?;

        Ok(id)
    }

    async fn list(&self, req: ListRequest) -> anyhow::Result<Vec<OrderItem>> {
        let mut conn = self.session.lock().await;

        let order_item_cols = vec![
            (OrderItems::Table, OrderItems::Id),
            (OrderItems::Table, OrderItems::Quantity),
            (OrderItems::Table, OrderItems::Status),
            (OrderItems::Table, OrderItems::CreatedAt),
            (OrderItems::Table, OrderItems::UpdatedAt),
            (OrderItems::Table, OrderItems::DeletedAt),
            (OrderItems::Table, OrderItems::CustomerId),
            (OrderItems::Table, OrderItems::ProductId),
        ];
        let customer_cols = vec![
            (Customers::Table, Customers::Name),
            (Customers::Table, Customers::CreatedAt),
        ];
        let product_cols = vec![
            (Products::Table, Products::Name),
            (Products::Table, Products::Currency),
            (Products::Table, Products::Price),
            (Products::Table, Products::CreatedAt),
        ];

        let limit = req.page_size;
        let offset = req.page * req.page_size;
        let query = req.query.map(|q| format!("%{}%", q));

        let sql = Query::select()
            .columns(order_item_cols)
            .columns(customer_cols)
            .columns(product_cols)
            .from(OrderItems::Table)
            .join(
                JoinType::InnerJoin,
                Customers::Table,
                Expr::tbl(OrderItems::Table, OrderItems::CustomerId)
                    .equals(Customers::Table, Customers::Id),
            )
            .join(
                JoinType::InnerJoin,
                Products::Table,
                Expr::tbl(OrderItems::Table, OrderItems::ProductId)
                    .equals(Products::Table, Products::Id),
            )
            .cond_where(
                Cond::any()
                    .add_option(
                        query
                            .clone()
                            .map(|q| Expr::tbl(Customers::Table, Customers::Name).like(&q)),
                    )
                    .add_option(
                        query
                            .clone()
                            .map(|q| Expr::tbl(Customers::Table, Customers::Phone).like(&q)),
                    )
                    .add_option(query.map(|q| Expr::tbl(Products::Table, Products::Name).like(&q))),
            )
            .limit(limit)
            .offset(offset)
            .to_string(PostgresQueryBuilder);

        Ok(sqlx::query_as::<_, OrderItem>(&sql)
            .fetch_all(conn.deref_mut())
            .await?)
    }

    async fn update(&self, req: UpdateOrderItemRequest) -> anyhow::Result<bool> {
        let mut conn = self.session.lock().await;

        let mut update_values = vec![];

        if let Some(customer_id) = req.customer_id {
            update_values.push((OrderItems::CustomerId, customer_id.into()));
        }

        if let Some(product_id) = req.product_id {
            update_values.push((OrderItems::ProductId, product_id.into()));
        }

        if let Some(quantity) = req.quantity {
            update_values.push((OrderItems::Quantity, quantity.into()));
        }

        if let Some(status) = req.status {
            update_values.push((OrderItems::Status, status.into()));
        }

        if update_values.is_empty() {
            return Ok(false);
        }

        let sql = Query::update()
            .table(OrderItems::Table)
            .values(update_values)
            .and_where(Expr::tbl(OrderItems::Table, OrderItems::Id).eq(req.id))
            .to_string(PostgresQueryBuilder);

        Ok(sqlx::query(&sql)
            .execute(conn.deref_mut())
            .await
            .map(|e| e.rows_affected() > 0)?)
    }

    async fn update_items_status(
        &self,
        req: UpdateOrderItemsStatusRequest,
    ) -> anyhow::Result<bool> {
        let mut conn = self.session.lock().await;

        let sql = Query::update()
            .table(OrderItems::Table)
            .values(vec![(OrderItems::Status, req.status.into())])
            .and_where(Expr::tbl(OrderItems::Table, OrderItems::Id).is_in(req.ids))
            .to_string(PostgresQueryBuilder);

        Ok(sqlx::query(&sql)
            .execute(conn.deref_mut())
            .await
            .map(|e| e.rows_affected() > 0)?)
    }
}

#[async_trait]
impl ProductRepo for ProductRepoImpl {
    async fn get(&self, id: u64) -> anyhow::Result<Option<Product>> {
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
}

#[async_trait]
impl CustomerRepo for CustomerRepoImpl {
    async fn get(&self, id: u64) -> anyhow::Result<Option<Customer>> {
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

        Ok(sqlx::query_as::<_, Customer>(&sql)
            .fetch_optional(conn.deref_mut())
            .await?)
    }
}
