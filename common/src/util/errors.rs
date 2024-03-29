use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Error)]
pub enum AppError {
    #[error("database error: {0}")]
    DatabaseError(String),
    #[error("try to make a connection, but it failed reason: {0}")]
    ConnectionError(String),
    #[error("{0}")]
    BadRequest(String),
}
