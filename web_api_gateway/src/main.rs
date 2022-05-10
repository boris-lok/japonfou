use anyhow::Result;
use tonic::transport::Endpoint;
use warp::Filter;

use common::config::base_config::Config;
use common::config::postgres_config::PostgresConfig;
use common::pb::customer_services_client::CustomerServicesClient;
use common::pb::product_services_client::ProductServicesClient;
use common::util::connections::create_database_connection;
use common::util::tools::tracing_initialize;

use crate::util::env::Env;
use crate::util::recover::rejection_handler;

mod customer;
mod product;
mod util;

#[tokio::main]
async fn main() -> Result<()> {
    let env_file = concat!(env!("CARGO_MANIFEST_DIR"), "/", "env", "/", "dev.env");
    let _ = dotenv::from_path(env_file);

    let config = Config::new();
    tracing_initialize(config.debug, "logs/", "gateway");

    let postgres = PostgresConfig::new();
    let database_connection_pool = create_database_connection(postgres)
        .await
        .expect("Can create a database connection pool.");

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Access-Control-Allow-Origin", "Content-Type"])
        .allow_credentials(true)
        .expose_headers(vec!["set-cookie"])
        .allow_methods(vec!["GET", "POST", "DELETE", "PUT", "PATCH"]);

    let addr = dotenv::var("CUSTOMER_HOST_ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1:10001".to_string())
        .parse::<Endpoint>()
        .expect("Can't parse hosting address.");

    let grpc_customer_client = CustomerServicesClient::connect(addr).await.unwrap();

    let addr = dotenv::var("PRODUCT_HOST_ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1:10002".to_string())
        .parse::<Endpoint>()
        .expect("Can't parse hosting address.");

    let grpc_product_client = ProductServicesClient::connect(addr).await.unwrap();

    let env = Env::new(true, grpc_customer_client, grpc_product_client);

    let routes = customer::routes::routes(env.clone())
        .with(cors)
        .with(warp::trace::request())
        .recover(rejection_handler);

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;

    database_connection_pool.close().await;

    Ok(())
}
