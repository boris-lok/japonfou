use chrono::{DateTime, NaiveDateTime, Utc};
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
