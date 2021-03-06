use std::convert::Infallible;
use std::error::Error;

use serde::Serialize;
use tonic::Status;
use tracing::log::error;
use warp::http::StatusCode;
use warp::Reply;

use common::util::errors::AppError;

use crate::util::error::ServerError;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}

impl From<(u16, String)> for ErrorResponse {
    fn from(t: (u16, String)) -> Self {
        Self {
            code: t.0,
            message: t.1,
        }
    }
}

pub fn custom_error_handler(status: Status) -> warp::reject::Rejection {
    let msg = status.message();
    tracing::error!(message = msg);
    warp::reject::custom(ServerError::Reason(msg.to_string()))
}

pub async fn rejection_handler(err: warp::Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    error!("unhandled rejection: {:?}", err);

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "not found.".to_string();
    } else if let Some(AppError::DatabaseError(s)) = err.find() {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = s.to_string();
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = (match e.source() {
            Some(cause) => {
                if cause.to_string().contains("denom") {
                    "field_error: denom"
                } else {
                    "bad request."
                }
            }
            _ => "bad request.",
        })
        .to_string()
    } else if let Some(ServerError::Reason(s)) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = s.to_string();
    } else if let Some(ServerError::Other(e)) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = e.to_string();
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "unhandled rejection.".to_string();
    }

    let response: ErrorResponse = (code.as_u16(), message).into();

    let json = warp::reply::json(&response);

    Ok(warp::reply::with_status(json, code))
}
