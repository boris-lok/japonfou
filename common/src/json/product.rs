use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::pb;
use crate::util::tools::timestamp2datetime;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub currency: i16,
    pub price: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<Product> for pb::Product {
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

impl From<pb::Product> for Product {
    fn from(p: pb::Product) -> Self {
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