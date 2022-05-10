use warp::{Filter, filters::BoxedFilter, Reply};

use crate::customer::handlers::v1::{create, get, list, update};
use crate::customer::json::ListCustomerRequest;
use crate::util::env::Env;
use crate::util::middleware::with_env::with_env;

pub fn routes(env: Env) -> BoxedFilter<(impl Reply,)> {
    let get_route = warp::path!("api" / "v1" / "customers" / u64)
        .and(warp::get())
        .and(with_env(env.clone()))
        .and_then(get);

    let create_route = warp::path!("api" / "v1" / "customers")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_env(env.clone()))
        .and_then(create);

    let update_route = warp::path!("api" / "v1" / "customers")
        .and(warp::put())
        .and(warp::body::json())
        .and(with_env(env.clone()))
        .and_then(update);

    let list_route = warp::path!("api" / "v1" / "customers")
        .and(warp::get())
        .and(warp::query::<ListCustomerRequest>())
        .and(with_env(env))
        .and_then(list);

    let routes = get_route.or(create_route).or(update_route).or(list_route);

    routes.boxed()
}
