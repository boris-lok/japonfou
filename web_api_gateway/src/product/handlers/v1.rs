use warp::reply::Reply;

use common::json::product::Product;
use common::pb;

use crate::product::json::{CreateProductRequest, ListProductRequest, UpdateProductRequest};
use crate::util::alias::WebResult;
use crate::util::env::Env;
use crate::util::recover::custom_error_handler;

pub async fn get(req: u64, env: Env) -> WebResult<impl Reply> {
    let mut client = env.grpc_product_client;

    client
        .get(pb::GetProductRequest { id: req })
        .await
        .map(|p| {
            let product = p.into_inner().product;
            match product {
                None => warp::reply::reply().into_response(),
                Some(c) => {
                    let c: Product = c.into();
                    warp::reply::json(&c).into_response()
                }
            }
        })
        .map_err(custom_error_handler)
}

pub async fn create(req: CreateProductRequest, env: Env) -> WebResult<impl Reply> {
    let mut client = env.grpc_product_client;

    let req: pb::CreateProductRequest = req.into();

    client
        .create(req)
        .await
        .map(|p| {
            let p: Product = p.into_inner().into();
            warp::reply::json(&p)
        })
        .map_err(custom_error_handler)
}

pub async fn update(req: UpdateProductRequest, env: Env) -> WebResult<impl Reply> {
    let mut client = env.grpc_product_client;

    let req: pb::UpdateProductRequest = req.into();

    client
        .update(req)
        .await
        .map(|p| {
            let p: Product = p.into_inner().into();
            warp::reply::json(&p)
        })
        .map_err(custom_error_handler)
}

pub async fn list(req: ListProductRequest, env: Env) -> WebResult<impl Reply> {
    let mut client = env.grpc_product_client;

    let req: pb::ListProductRequest = req.into();

    client
        .list(req)
        .await
        .map(|c| {
            let c = c
                .into_inner()
                .products
                .into_iter()
                .map(|c| c.into())
                .collect::<Vec<Product>>();
            warp::reply::json(&c)
        })
        .map_err(custom_error_handler)
}
