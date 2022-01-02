use std::convert::Infallible;
use std::error::Error;
use tracing::error;

use crate::core::error::{AppError, ErrorResponse};
use warp::http::StatusCode;
use warp::{Filter, Reply};

use crate::Environment;

pub fn with_env(
    env: Environment,
) -> impl Filter<Extract = (Environment,), Error = Infallible> + Clone {
    warp::any().map(move || env.clone())
}

pub async fn rejection_handler(err: warp::Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    error!("unhandled rejection: {:?}", err);

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "not found.";
    } else if let Some(AppError::AuthorizeFailed) = err.find() {
        code = StatusCode::UNAUTHORIZED;
        message = "un-authorized.";
    } else if let Some(AppError::DecodeClaimsFailed) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = "decode claims failed.";
    } else if let Some(AppError::DatabaseError) = err.find() {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "database error.";
    } else if let Some(AppError::HashPasswordFailed) = err.find() {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "hash password failed.";
    } else if let Some(AppError::UserNotExist) = err.find() {
        code = StatusCode::NOT_FOUND;
        message = "user not exist.";
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
