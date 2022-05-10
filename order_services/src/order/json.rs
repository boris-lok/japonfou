use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Deserialize, Debug)]
pub struct CreateOrderRequest {
    pub customer_id: u64,
    pub product_ids: Vec<u64>,
    pub quantity: u16,
    pub status: OrderStatus,
}

#[derive(Debug, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum OrderStatus {
    Picked = 0,
    Available = 1,
    Ordering = 2,
    OutOfStock = 3,
}
