use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    AuthorizeFailed,
    DecodeClaimsFailed(jsonwebtoken::errors::Error),
    NotFound,
    DatabaseError(sqlx::Error),
    CreateUserFailed,
    HashPasswordFailed,
    UserNotExist,
}

impl PartialEq for AppError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&AppError::AuthorizeFailed, &AppError::AuthorizeFailed) => true,
            (&AppError::DecodeClaimsFailed(_), &AppError::DecodeClaimsFailed(_)) => true,
            (&AppError::NotFound, &AppError::NotFound) => true,
            (&AppError::DatabaseError(_), &AppError::DatabaseError(_)) => true,
            (&AppError::CreateUserFailed, &AppError::CreateUserFailed) => true,
            (&AppError::HashPasswordFailed, &AppError::HashPasswordFailed) => true,
            (&AppError::UserNotExist, &AppError::UserNotExist) => true,
            (_, _) => false,
        }
    }
}

impl warp::reject::Reject for AppError {}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    code: u16,
    message: String,
}

impl From<(u16, &str)> for ErrorResponse {
    fn from(e: (u16, &str)) -> Self {
        Self {
            code: e.0,
            message: e.1.to_string(),
        }
    }
}
