use async_trait::async_trait;

use sea_query::{Expr, JoinType, PostgresQueryBuilder, Query};

use common::json::customer::Customers;
use common::json::order_item::{OrderItem, OrderItems};
use common::json::product::Products;
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
}
