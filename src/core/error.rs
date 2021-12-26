#[derive(Debug)]
pub enum AppError {
    AuthorizeFailed,
    DecodeClaimsFailed(jsonwebtoken::errors::Error),
    NotFound,
    DatabaseError(sqlx::Error),
}

impl PartialEq for AppError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&AppError::AuthorizeFailed, &AppError::AuthorizeFailed) => true,
            (&AppError::DecodeClaimsFailed(_), &AppError::DecodeClaimsFailed(_)) => true,
            (&AppError::NotFound, &AppError::NotFound) => true,
            (&AppError::DatabaseError(_), &AppError::DatabaseError(_)) => true,
            (_, _) => false,
        }
    }
}
