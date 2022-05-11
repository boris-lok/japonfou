use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Deserialize, Debug)]
pub struct CreateOrderItemRequest {
    pub customer_id: u64,
    pub product_id: u64,
    pub quantity: u16,
    pub status: OrderItemStatus,
}

impl From<CreateOrderItemRequest> for common::order_item_pb::CreateOrderItemRequest {
    fn from(r: CreateOrderItemRequest) -> Self {
        Self {
            customer_id: r.customer_id,
            product_id: r.product_id,
            quantity: r.quantity as u32,
            status: r.status as u32,
        }
    }
}

#[derive(Debug, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum OrderItemStatus {
    Picked = 0,
    Available = 1,
    Ordering = 2,
    OutOfStock = 3,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrderItemRequest {
    pub id: u64,
    pub customer_id: Option<u64>,
    pub product_id: Option<u64>,
    pub quantity: Option<u16>,
    pub status: Option<OrderItemStatus>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrderItemStatusRequest {
    pub ids: Vec<u64>,
    pub status: OrderItemStatus,
}

#[derive(Debug, Deserialize)]
pub struct ListOrderItemsRequest {
    pub query: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}
