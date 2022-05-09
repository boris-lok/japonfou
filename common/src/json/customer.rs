use chrono::{DateTime, Utc};
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::pb;
use crate::util::tools::timestamp2datetime;

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct Customer {
    pub id: i64,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<Customer> for pb::Customer {
    fn from(c: Customer) -> Self {
        Self {
            id: c.id as u64,
            name: c.name,
            email: c.email,
            phone: c.phone,
            created_at: c.created_at.timestamp() as u64,
            updated_at: c.updated_at.map(|d| d.timestamp() as u64),
        }
    }
}

impl From<pb::Customer> for Customer {
    fn from(c: pb::Customer) -> Self {
        Self {
            id: c.id as i64,
            name: c.name,
            email: c.email,
            phone: c.phone,
            created_at: timestamp2datetime(c.created_at),
            updated_at: c.updated_at.map(timestamp2datetime),
        }
    }
}

#[derive(Iden, Clone)]
pub enum Customers {
    Table,
    Id,
    Name,
    Email,
    Phone,
    CreatedAt,
    UpdatedAt,
}
