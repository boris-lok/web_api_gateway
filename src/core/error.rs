use sqlx::Error;

#[derive(Debug)]
pub enum AppError {
    AuthorizeFailed,
    NotFound,
    DatabaseError(Error)
}