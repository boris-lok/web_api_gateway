

use std::str::FromStr;
use std::sync::Arc;

use jsonwebtoken::{decode, DecodingKey, Validation};

use uuid::Uuid;
use warp::{Filter, Rejection};
use warp::http::{HeaderMap, HeaderValue};

use crate::{Environment, WebResult};
use crate::auth::json::claims::Claims;
use crate::auth::repo::AuthRepository;
use crate::core::error::{AppError};

const BEARER: &str = "Bearer ";

pub fn authenticated(
    env: Environment,
) -> impl Filter<Extract = (Claims,), Error = Rejection> + Clone {
    warp::header::headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| headers)
        .and(warp::any().map(move || env.clone()))
        .and_then(authorize)
}

async fn authorize(headers: HeaderMap<HeaderValue>, env: Environment) -> WebResult<Claims> {
    match jwt_from_header(&headers) {
        Ok(jwt) => {
            let validation_config = Validation {
                validate_exp: false,
                ..Default::default()
            };

            let claims = decode::<Claims>(
                jwt.as_str(),
                &DecodingKey::from_secret(env.config.secret_key.as_bytes()),
                &validation_config,
            );

            if claims.is_err() {
                return Err(warp::reject::custom(AppError::AuthorizeFailed));
            }

            let claims = claims.unwrap().claims;

            if check_is_expired(&claims, env.auth_repo) {
                return Err(warp::reject::custom(AppError::TokenIsExpired));
            }

            Ok(claims)
        }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

fn check_is_expired(claims: &Claims, auth_repo: Arc<impl AuthRepository>) -> bool {
    Uuid::from_str(claims.sub.as_str())
        .map(|uuid| auth_repo.get(uuid))
        .ok()
        .flatten()
        .is_none()
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
