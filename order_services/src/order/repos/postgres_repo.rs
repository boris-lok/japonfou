use async_trait::async_trait;
use futures::FutureExt;

use sea_query::{Expr, JoinType, PostgresQueryBuilder, Query};

use crate::ID_GENERATOR;
use common::json::customer::Customers;
use common::json::order_item::{OrderItem, OrderItems};
use common::json::product::Products;
use common::order_item_pb::CreateOrderItemRequest;
use common::util::alias::PostgresAcquire;

use crate::order::repos::repo::OrderItemRepo;

pub struct OrderItemRepoImpl;

#[async_trait]
impl OrderItemRepo for OrderItemRepoImpl {
    async fn get(
        &self,
        id: u64,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> anyhow::Result<Option<OrderItem>> {
        let mut conn = executor.acquire().await.unwrap();

        let order_item_cols = vec![
            OrderItems::Id,
            OrderItems::Quantity,
            OrderItems::Status,
            OrderItems::CreatedAt,
            OrderItems::UpdatedAt,
            OrderItems::DeletedAt,
            OrderItems::CustomerId,
            OrderItems::ProductId,
        ];
        let customer_cols = vec![Customers::Name, Customers::CreatedAt];
        let product_cols = vec![
            Products::Name,
            Products::Currency,
            Products::Price,
            Products::CreatedAt,
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
            .and_where(Expr::col(OrderItems::Id).eq(id))
            .to_string(PostgresQueryBuilder);

        Ok(sqlx::query_as::<_, OrderItem>(&sql)
            .fetch_optional(&mut *conn)
            .await?)
    }

    async fn create(
        &self,
        req: CreateOrderItemRequest,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> anyhow::Result<u64> {
        let mut conn = executor.acquire().await.unwrap();

        let id = async move { ID_GENERATOR.clone().lock().unwrap().next_id() }
            .boxed()
            .await as u64;

        let sql = Query::insert()
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

        let _ = sqlx::query(&sql).execute(&mut *conn).await?;

        Ok(id)
    }
}
