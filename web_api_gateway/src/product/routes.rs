use warp::{Filter, filters::BoxedFilter, Reply};

use crate::product::handlers::v1::{create, get, list, update};
use crate::product::json::ListProductRequest;
use crate::util::env::Env;
use crate::util::middleware::with_env::with_env;

pub fn routes(env: Env) -> BoxedFilter<(impl Reply,)> {
    let get_route = warp::path!("api" / "v1" / "products" / u64)
        .and(warp::get())
        .and(with_env(env.clone()))
        .and_then(get);

    let create_route = warp::path!("api" / "v1" / "products")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_env(env.clone()))
        .and_then(create);

    let update_route = warp::path!("api" / "v1" / "products")
        .and(warp::put())
        .and(warp::body::json())
        .and(with_env(env.clone()))
        .and_then(update);

    let list_route = warp::path!("api" / "v1" / "products")
        .and(warp::get())
        .and(warp::query::<ListProductRequest>())
        .and(with_env(env))
        .and_then(list);

    let routes = get_route.or(create_route).or(update_route).or(list_route);

    routes.boxed()
}
