use chrono::{DateTime, Utc};
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{Error, Row};

use crate::json::customer::Customer;
use crate::json::product::Product;
use crate::order_item_pb;
use crate::util::tools::timestamp2datetime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderItem {
    pub id: i64,
    pub customer: Customer,
    pub product: Product,
    pub quantity: u32,
    pub status: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl<'r> ::sqlx::FromRow<'r, PgRow> for OrderItem {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        println!("{:?}", row.columns());
        // let id: i64 = row.try_get("id")?;
        Err(Error::RowNotFound)
        // let quantity: u32 = row.try_get("quantity")?;
        // let status: u32 = row.try_get("status")?;
        // let created_at: DateTime<Utc> = row.try_get("created_at")?;
        // let updated_at: Option<DateTime<Utc>> = row.try_get("updated_at")?;
        // let deleted_at: Option<DateTime<Utc>> = row.try_get("deleted_at")?;
    }
}

impl From<OrderItem> for order_item_pb::OrderItem {
    fn from(o: OrderItem) -> Self {
        Self {
            id: o.id as u64,
            product: Some(o.product.into()),
            customer: Some(o.customer.into()),
            quantity: o.quantity,
            created_at: o.created_at.timestamp() as u64,
            updated_at: o.updated_at.map(|d| d.timestamp() as u64),
            deleted_at: o.deleted_at.map(|d| d.timestamp() as u64),
            status: o.status,
        }
    }
}

impl From<order_item_pb::OrderItem> for OrderItem {
    fn from(o: order_item_pb::OrderItem) -> Self {
        Self {
            id: o.id as i64,
            customer: o.customer.map(|c| c.into()).unwrap(),
            product: o.product.map(|p| p.into()).unwrap(),
            quantity: o.quantity,
            status: o.status,
            created_at: timestamp2datetime(o.created_at),
            updated_at: o.updated_at.map(timestamp2datetime),
            deleted_at: o.deleted_at.map(timestamp2datetime),
        }
    }
}

#[derive(Iden, Clone)]
pub enum OrderItems {
    Table,
    Id,
    CustomerId,
    ProductId,
    Quantity,
    Status,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
