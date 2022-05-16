use warp::{filters::BoxedFilter, Filter, Reply};

use crate::order::handlers::v1::{create, get, list, update};
use crate::order::json::ListOrderItemsRequest;
use crate::util::env::Env;
use crate::util::middleware::with_env::with_env;

pub fn routes(env: Env) -> BoxedFilter<(impl Reply,)> {
    let get_route = warp::path!("api" / "v1" / "orders" / u64)
        .and(warp::get())
        .and(with_env(env.clone()))
        .and_then(get);

    let create_route = warp::path!("api" / "v1" / "orders")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_env(env.clone()))
        .and_then(create);

    let list_route = warp::path!("api" / "v1" / "orders")
        .and(warp::get())
        .and(warp::query::<ListOrderItemsRequest>())
        .and(with_env(env.clone()))
        .and_then(list);

    let update_route = warp::path!("api" / "v1" / "orders")
        .and(warp::put())
        .and(warp::body::json())
        .and(with_env(env))
        .and_then(update);

    let routes = get_route.or(create_route).or(list_route).or(update_route);

    routes.boxed()
}
