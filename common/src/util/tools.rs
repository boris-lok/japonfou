use crate::util::errors::AppError;
use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use futures::lock::Mutex;
use sqlx::pool::PoolConnection;
use sqlx::Postgres;
use std::ops::DerefMut;
use std::sync::Arc;
use tonic::Status;
use tracing_subscriber::fmt::writer::MakeWriterExt;

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
