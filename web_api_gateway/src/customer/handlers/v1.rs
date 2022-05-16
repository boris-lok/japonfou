use warp::reply::Reply;

use common::json::customer::Customer;
use common::{customer_pb, types};

use crate::customer::json::{CreateCustomerRequest, ListCustomerRequest, UpdateCustomerRequest};
use crate::util::alias::WebResult;
use crate::util::env::Env;

use crate::util::recover::custom_error_handler;

pub(crate) async fn get(req: u64, env: Env) -> WebResult<impl Reply> {
    let mut client = env.grpc_customer_client;

    client
        .get(types::GetByIdRequest { id: req })
        .await
        .map(|c| {
            let customer = c.into_inner().customer;
            match customer {
                None => warp::reply::reply().into_response(),
                Some(c) => {
                    let c: Customer = c.into();
                    warp::reply::json(&c).into_response()
                }
            }
        })
        .map_err(custom_error_handler)
}

pub(crate) async fn create(req: CreateCustomerRequest, env: Env) -> WebResult<impl Reply> {
    let mut client = env.grpc_customer_client;

    let req: customer_pb::CreateCustomerRequest = req.into();

    client
        .create(req)
        .await
        .map(|c| {
            let c: Customer = c.into_inner().into();
            warp::reply::json(&c)
        })
        .map_err(custom_error_handler)
}

pub(crate) async fn update(req: UpdateCustomerRequest, env: Env) -> WebResult<impl Reply> {
    let mut client = env.grpc_customer_client;

    let req: customer_pb::UpdateCustomerRequest = req.into();

    client
        .update(req)
        .await
        .map(|c| {
            let c: Customer = c.into_inner().into();
            warp::reply::json(&c)
        })
        .map_err(custom_error_handler)
}

pub(crate) async fn list(req: ListCustomerRequest, env: Env) -> WebResult<impl Reply> {
    let mut client = env.grpc_customer_client;

    let req: types::ListRequest = req.into();

    client
        .list(req)
        .await
        .map(|c| {
            let c = c
                .into_inner()
                .customers
                .into_iter()
                .map(|c| c.into())
                .collect::<Vec<Customer>>();
            warp::reply::json(&c)
        })
        .map_err(custom_error_handler)
}
