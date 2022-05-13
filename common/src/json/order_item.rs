use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
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
        // TODO: it's not good to use index to get the value. maybe there has another way to do this.
        let id: i64 = row.try_get(0)?;
        let quantity: i16 = row.try_get(1)?;
        let status: i16 = row.try_get(2)?;
        let created_at: DateTime<Utc> = row.try_get(3)?;
        let updated_at: Option<DateTime<Utc>> = row.try_get(4)?;
        let deleted_at: Option<DateTime<Utc>> = row.try_get(5)?;

        let customer_id: i64 = row.try_get(6)?;
        let product_id: i64 = row.try_get(7)?;

        let customer_name: String = row.try_get(8)?;
        let customer_created_at: DateTime<Utc> = row.try_get(9)?;

        let product_name: String = row.try_get(10)?;
        let product_currency: i16 = row.try_get(11)?;
        let product_price: Decimal = row.try_get(12)?;
        let product_created_at: DateTime<Utc> = row.try_get(13)?;

        let product = Product {
            id: product_id,
            name: product_name,
            currency: product_currency,
            price: product_price,
            created_at: product_created_at,
            updated_at: None,
            deleted_at: None,
        };

        let customer = Customer {
            id: customer_id,
            name: customer_name,
            email: None,
            phone: None,
            created_at: customer_created_at,
            updated_at: None,
        };

        Ok(Self {
            id,
            customer,
            product,
            quantity: quantity as u32,
            status: status as u32,
            created_at,
            updated_at,
            deleted_at,
        })
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
