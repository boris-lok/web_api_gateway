use serde::Serialize;

#[derive(Debug, PartialEq)]
pub enum AppError {
    AuthorizeFailed,
    DatabaseError,
    HashPasswordFailed,
    UserNotExist,
    TokenNotExist,
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
