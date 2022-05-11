use warp::Reply;

use common::json::order_item::OrderItem;
use common::{order_item_pb, types};

use crate::order::json::CreateOrderItemRequest;
use crate::util::alias::WebResult;
use crate::util::recover::custom_error_handler;
use crate::Env;

pub(crate) async fn get(req: u64, env: Env) -> WebResult<impl Reply> {
    let mut client = env.grpc_order_client;

    client
        .get(types::GetByIdRequest { id: req })
        .await
        .map(|c| {
            let item = c.into_inner().item;
            match item {
                None => warp::reply::reply().into_response(),
                Some(c) => {
                    let c: OrderItem = c.into();
                    warp::reply::json(&c).into_response()
                }
            }
        })
        .map_err(custom_error_handler)
}

pub(crate) async fn create(req: CreateOrderItemRequest, env: Env) -> WebResult<impl Reply> {
    let mut client = env.grpc_order_client;

    let req: order_item_pb::CreateOrderItemRequest = req.into();

    client
        .create(req)
        .await
        .map(|o| {
            let o: common::json::order_item::OrderItem = o.into_inner().into();
            warp::reply::json(&o)
        })
        .map_err(custom_error_handler)
}
