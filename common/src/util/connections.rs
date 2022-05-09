use anyhow::Result;
use r2d2_redis::{r2d2, RedisConnectionManager};
use snowflake::SnowflakeGenerator;
use sqlx::{Pool, Postgres};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use crate::config::id_generator_config::IdGeneratorConfig;
use crate::config::postgres_config::PostgresConfig;
use crate::config::redis_config::RedisConfig;

/// Create a database connection.
///
/// return: database connection pool.
pub async fn create_database_connection(config: PostgresConfig) -> Result<Pool<Postgres>> {
    let connection_options = PgConnectOptions::new()
        .host(&config.host)
        .database(&config.database)
        .username(&config.username)
        .password(&config.password)
        .port(config.port);

    let pool = PgPoolOptions::new()
        .max_connections(config.max_connection as u32)
        .connect_with(connection_options)
        .await?;

    Ok(pool)
}

/// Create a redis connection
///
/// return: redis connection pool.
pub async fn create_redis_connection(
    config: RedisConfig,
) -> Result<r2d2::Pool<RedisConnectionManager>> {
    let redis_uri = format!(
        "redis://{}:{}@{}:{}",
        config.username, config.password, config.host, config.port
    );

    let manager = RedisConnectionManager::new(redis_uri)?;
    let pool = r2d2::Pool::builder().build(manager)?;

    Ok(pool)
}

/// Create a id generator
///
/// return SnowflakeGenerator.
pub fn create_id_generator(config: IdGeneratorConfig) -> SnowflakeGenerator {
    SnowflakeGenerator::new(
        config.worker_id,
        config.data_center_id,
        config.timestamp_offset,
    )
}
