#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error("failed to process: {0}")]
    Reason(String),
}

impl warp::reject::Reject for ServerError {}
