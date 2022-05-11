use chrono::{DateTime, Utc};
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::util::tools::timestamp2datetime;
use crate::{order_item_pb, product_pb};

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub currency: i16,
    pub price: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<Product> for product_pb::Product {
    fn from(p: Product) -> Self {
        Self {
            id: p.id as u64,
            name: p.name,
            currency: p.currency as i32,
            price: p.price.to_f64().unwrap(),
            created_at: p.created_at.timestamp() as u64,
            updated_at: p.updated_at.map(|d| d.timestamp() as u64),
            deleted_at: p.deleted_at.map(|d| d.timestamp() as u64),
        }
    }
}

impl From<product_pb::Product> for Product {
    fn from(p: product_pb::Product) -> Self {
        Self {
            id: p.id as i64,
            name: p.name,
            currency: p.currency as i16,
            price: Decimal::from_f64(p.price).unwrap(),
            created_at: timestamp2datetime(p.created_at),
            updated_at: p.updated_at.map(timestamp2datetime),
            deleted_at: p.deleted_at.map(timestamp2datetime),
        }
    }
}

impl From<Product> for order_item_pb::order_item::Product {
    fn from(p: Product) -> Self {
        Self {
            id: p.id as u64,
            name: p.name,
            currency: p.currency as u32,
            price: p.price.to_f64().unwrap(),
            created_at: p.created_at.timestamp() as u64,
        }
    }
}

impl From<order_item_pb::order_item::Product> for Product {
    fn from(p: order_item_pb::order_item::Product) -> Self {
        Self {
            id: p.id as i64,
            name: p.name,
            currency: p.currency as i16,
            price: Decimal::from_f64(p.price).unwrap(),
            created_at: timestamp2datetime(p.created_at),
            updated_at: None,
            deleted_at: None,
        }
    }
}

#[derive(Iden, Clone)]
pub enum Products {
    Table,
    Id,
    Name,
    Currency,
    Price,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
