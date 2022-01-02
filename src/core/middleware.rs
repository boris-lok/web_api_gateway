use std::convert::Infallible;
use std::error::Error;

use jsonwebtoken::{decode, DecodingKey, Validation};
use tracing::error;
use warp::{Filter, Rejection, Reply};
use warp::http::{HeaderMap, HeaderValue, StatusCode};

use crate::{Environment, WebResult};
use crate::auth::json::Claims;
use crate::core::error::{AppError, ErrorResponse};

const BEARER: &str = "Bearer ";

pub fn with_env(
    env: Environment,
) -> impl Filter<Extract = (Environment,), Error = Infallible> + Clone {
    warp::any().map(move || env.clone())
}

pub fn authenticated(
    secret_key: String,
) -> impl Filter<Extract = (Claims,), Error = Rejection> + Clone {
    warp::header::headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| headers)
        .and(warp::any().map(move || secret_key.clone()))
        .and_then(authorize)
}

async fn authorize(headers: HeaderMap<HeaderValue>, secret_key: String) -> WebResult<Claims> {
    match jwt_from_header(&headers) {
        Ok(jwt) => {
            let claims = decode::<Claims>(
                jwt.as_str(),
                &DecodingKey::from_secret(secret_key.as_bytes()),
                &Validation::default(),
            );

            if claims.is_err() {
                return Err(warp::reject::custom(AppError::AuthorizeFailed));
            }

            let claims = claims.unwrap().claims;
            Ok(claims)
        }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String, AppError> {
    let header = headers
        .get(warp::http::header::AUTHORIZATION)
        .map(|value| std::str::from_utf8(value.as_bytes()).ok())
        .flatten();

    return match header {
        None => Err(AppError::TokenNotExist),
        Some(value) => {
            if !value.starts_with(BEARER) {
                return Err(AppError::TokenNotExist);
            }
            Ok(value.trim_start_matches(BEARER).to_owned())
        }
    };
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
