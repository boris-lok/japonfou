use serde::Deserialize;

use common::pb;

#[derive(Debug, Deserialize)]
pub struct CreateCustomerRequest {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

impl From<CreateCustomerRequest> for pb::CreateCustomerRequest {
    fn from(c: CreateCustomerRequest) -> Self {
        Self {
            name: c.name,
            email: c.email,
            phone: c.phone,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateCustomerRequest {
    pub id: u64,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

impl From<UpdateCustomerRequest> for pb::UpdateCustomerRequest {
    fn from(c: UpdateCustomerRequest) -> Self {
        Self {
            id: c.id,
            name: c.name,
            email: c.email,
            phone: c.phone,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ListCustomerRequest {
    pub query: Option<String>,
    pub cursor: Option<u64>,
    pub page_size: Option<u32>,
}

impl From<ListCustomerRequest> for pb::ListCustomerRequest {
    fn from(e: ListCustomerRequest) -> Self {
        Self {
            query: e.query,
            cursor: e.cursor,
            page_size: e.page_size.unwrap_or(20),
        }
    }
}
