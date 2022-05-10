use serde::Deserialize;

use common::{pb, types};

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
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

impl From<ListCustomerRequest> for types::ListRequest {
    fn from(e: ListCustomerRequest) -> Self {
        Self {
            query: e.query,
            page_size: e.page_size.unwrap_or(20),
            page: e.page.unwrap_or(0),
        }
    }
}
