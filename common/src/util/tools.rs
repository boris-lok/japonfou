use std::ops::DerefMut;
use std::sync::Arc;

use crate::config::base_config::Config;
use crate::config::postgres_config::PostgresConfig;
use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use futures::lock::Mutex;
use sqlx::pool::PoolConnection;
use sqlx::Postgres;
use tonic::Status;
use tracing_subscriber::fmt::writer::MakeWriterExt;

use crate::util::errors::AppError;

/// Init tracing - show logs in console to create daily log files.
///
/// params:
/// - debug: If debug is true, the logs only show in console. Otherwise, it will create daily log file.
/// - dir: If debug is false, daily log files will store in this dir.
/// - prefix: log files prefix.
///
pub fn tracing_initialize(debug: bool, dir: &str, prefix: &str) {
    if debug {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    } else {
        let logfile = tracing_appender::rolling::daily(dir, prefix);
        let stdout = std::io::stdout.with_max_level(tracing::Level::INFO);

        tracing_subscriber::fmt()
            .with_writer(stdout.and(logfile))
            .init();
    }
}

/// convert timestamp to datetime.
///
/// params:
/// - timestamp: the unix timestamp.
///
/// return:
/// - datetime: the utc datetime.
pub fn timestamp2datetime(timestamp: u64) -> DateTime<Utc> {
    let native_datetime = NaiveDateTime::from_timestamp(timestamp as i64, 0);

    DateTime::<Utc>::from_utc(native_datetime, Utc)
}

/// handle app error and parse it to grpc status
///
/// params:
/// - err: app error.
///
/// return:
/// - Status
pub fn grpc_error_handler(err: AppError) -> Status {
    let msg = err.to_string();
    tracing::error!(message = msg.as_str());
    Status::failed_precondition(msg)
}

/// handle database error.
///
/// param:
/// - err: anyhow error.
///
/// return:
/// - AppError
pub fn database_error_handler(err: anyhow::Error) -> AppError {
    let msg = err.to_string();
    tracing::error!(message = msg.as_str());
    AppError::DatabaseError(msg)
}

/// begin a transaction
///
/// param:
/// - session: database connection session.
///
/// return:
/// - bool
pub async fn begin_transaction(session: Arc<Mutex<PoolConnection<Postgres>>>) -> Result<bool> {
    execute(session, "BEGIN;").await
}

/// commit a transaction
///
/// param:
/// - session: database connection session.
///
/// return:
/// - bool
pub async fn commit_transaction(session: Arc<Mutex<PoolConnection<Postgres>>>) -> Result<bool> {
    execute(session, "COMMIT;").await
}

/// rollback a transaction
///
/// param:
/// - session: database connection session.
///
/// return:
/// - bool
pub async fn rollback_transaction(session: Arc<Mutex<PoolConnection<Postgres>>>) -> Result<bool> {
    execute(session, "ROLLBACK;").await
}

/// execute a sql in database.
///
/// params:
/// - session: database connection session.
/// - sql
///
/// return:
/// - bool
async fn execute(session: Arc<Mutex<PoolConnection<Postgres>>>, sql: &str) -> Result<bool> {
    let mut conn = session.lock().await;

    Ok(sqlx::query(sql)
        .execute(conn.deref_mut())
        .await
        .map(|row| row.rows_affected() > 0)?)
}

/// Read postgresql config from the env.
///
/// host: read the POSTGRES_HOST value from the env.
/// database: read the POSTGRES_DATABASE value from the env.
/// username: read the POSTGRES_USERNAME value from the env.
/// password: read the POSTGRES_PASSWORD value from the env.
/// port: read the POSTGRES_PORT value from the env.
/// max_connection: read POSTGRES_MAX_CONNECTION value from the env.
///
/// return:
/// - PostgresConfig
pub fn read_postgresql_config_from_env() -> PostgresConfig {
    let host = dotenv::var("POSTGRES_HOST").expect("Can read the database host from env file.");

    let database =
        dotenv::var("POSTGRES_DATABASE").expect("Can read the database name from env file.");

    let username =
        dotenv::var("POSTGRES_USERNAME").expect("Can read the database username from env file.");

    let password =
        dotenv::var("POSTGRES_PASSWORD").expect("Can read the database password from env file.");

    let port = dotenv::var("POSTGRES_PORT")
        .expect("Can read the database port from env file.")
        .parse::<u16>()
        .expect("Can parse the database port from string to u16.");

    let max_connection = dotenv::var("POSTGRES_MAX_CONNECTION")
        .expect("Can read the database max connection from env file.")
        .parse::<u8>()
        .expect("Can parse the database max connection from string to u8.");

    PostgresConfig::new(
        host,
        database,
        username,
        password,
        Some(port),
        Some(max_connection),
    )
}

/// Read the default config from the env.
///
/// debug: read the DEBUG value from the env.
/// secret_key: read the SECRET_KEY value from the env.
///
/// return:
/// - Config
pub fn read_config_from_env() -> Config {
    let debug = dotenv::var("DEBUG")
        .map(|e| e.parse::<bool>().ok())
        .ok()
        .flatten();

    let secret_key = dotenv::var("SECRET_KEY").expect("Can read the secret key from the env file.");

    Config::new(debug, secret_key)
}
