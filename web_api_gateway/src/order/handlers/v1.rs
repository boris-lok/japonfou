use warp::Reply;

use common::json::order_item::OrderItem;
use common::types::ListRequest;
use common::{order_item_pb, types};

use crate::order::json::{
    CreateOrderItemRequest, ListOrderItemsRequest, UpdateOrderItemRequest,
    UpdateOrderItemStatusRequest,
};
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

pub(crate) async fn list(req: ListOrderItemsRequest, env: Env) -> WebResult<impl Reply> {
    let mut client = env.grpc_order_client;

    let req: ListRequest = req.into();

    client
        .list(req)
        .await
        .map(|items| {
            let items = items
                .into_inner()
                .items
                .into_iter()
                .map(|e| e.into())
                .collect::<Vec<OrderItem>>();
            warp::reply::json(&items)
        })
        .map_err(custom_error_handler)
}

pub(crate) async fn update(req: UpdateOrderItemRequest, env: Env) -> WebResult<impl Reply> {
    let mut client = env.grpc_order_client;

    let req: common::order_item_pb::UpdateOrderItemRequest = req.into();

    client
        .update(req)
        .await
        .map(|item| {
            let res: OrderItem = item.into_inner().into();
            warp::reply::json(&res)
        })
        .map_err(custom_error_handler)
}

pub(crate) async fn update_items_status(
    req: UpdateOrderItemStatusRequest,
    env: Env,
) -> WebResult<impl Reply> {
    let mut client = env.grpc_order_client;

    let req: common::order_item_pb::UpdateOrderItemsStatusRequest = req.into();

    client
        .update_order_items_status(req)
        .await
        .map(|_| warp::reply::reply())
        .map_err(custom_error_handler)
}
