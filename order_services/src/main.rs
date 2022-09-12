use std::sync::{Arc, Mutex};

use anyhow::Result;
use lazy_static::lazy_static;
use snowflake::SnowflakeGenerator;
use tonic::transport::Server;

use common::config::id_generator_config::IdGeneratorConfig;
use common::order_item_pb::order_services_server::OrderServicesServer;
use common::util::connections::{create_database_connection, create_id_generator};
use common::util::tools::{
    read_config_from_env, read_postgresql_config_from_env, tracing_initialize,
};

use crate::order::services::grpc_service::GrpcOrderServiceImpl;

mod order;

lazy_static! {
    static ref ID_GENERATOR: Arc<Mutex<SnowflakeGenerator>> = {
        let config = IdGeneratorConfig::new(0, 0, 0);
        let generator = create_id_generator(config);
        Arc::new(Mutex::new(generator))
    };
}

#[tokio::main]
async fn main() -> Result<()> {
    let env_file = concat!(env!("CARGO_MANIFEST_DIR"), "/", "env", "/", "dev.env");
    let _ = dotenv::from_path(env_file);

    let config = read_config_from_env();

    tracing_initialize(config.debug, "logs", "customers");

    let database_config = read_postgresql_config_from_env();

    let database_connection = create_database_connection(database_config)
        .await
        .expect("Can't connect to database.");

    let order_item_service = GrpcOrderServiceImpl::new(database_connection);

    let addr = dotenv::var("ORDER_HOST_ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1:10003".to_string())
        .parse()
        .expect("Can't parse hosting address.");

    tracing::info!(message = "starting customer server", %addr);

    Server::builder()
        .add_service(OrderServicesServer::new(order_item_service))
        .serve(addr)
        .await?;

    Ok(())
}
