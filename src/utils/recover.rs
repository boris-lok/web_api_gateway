use std::convert::Infallible;
use std::error::Error;

use serde::Serialize;
use tracing::log::error;
use warp::http::StatusCode;
use warp::Reply;

use common::utils::error::AppError;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}

impl From<(u16, &str)> for ErrorResponse {
    fn from(t: (u16, &str)) -> Self {
        Self {
            code: t.0,
            message: t.1.to_string(),
        }
    }
}

pub async fn rejection_handler(err: warp::Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    error!("unhandled rejection: {:?}", err);

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "not found.";
    } else if let Some(AppError::DatabaseError(s)) = err.find() {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = s;
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = match e.source() {
            Some(cause) => {
                if cause.to_string().contains("denom") {
                    "field_error: denom"
                } else {
                    "bad request."
                }
            }
            _ => "bad request.",
        }
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "unhandled rejection.";
    }

    let response: ErrorResponse = (code.as_u16(), message).into();

    let json = warp::reply::json(&response);

    Ok(warp::reply::with_status(json, code))
}
