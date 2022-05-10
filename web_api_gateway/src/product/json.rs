use serde::Deserialize;

use common::{pb, types};

#[derive(Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub currency: i32,
    pub price: f64,
}

impl From<CreateProductRequest> for pb::CreateProductRequest {
    fn from(r: CreateProductRequest) -> Self {
        Self {
            name: r.name,
            currency: r.currency,
            price: r.price,
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateProductRequest {
    pub id: u64,
    pub name: Option<String>,
    pub currency: Option<i32>,
    pub price: Option<f64>,
}

impl From<UpdateProductRequest> for pb::UpdateProductRequest {
    fn from(r: UpdateProductRequest) -> Self {
        Self {
            id: r.id,
            name: r.name,
            currency: r.currency,
            price: r.price,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ListProductRequest {
    pub query: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

impl From<ListProductRequest> for types::ListRequest {
    fn from(e: ListProductRequest) -> Self {
        Self {
            query: e.query,
            page: e.page.unwrap_or(0),
            page_size: e.page_size.unwrap_or(20),
        }
    }
}
