use sea_query::Iden;
use serde::Deserialize;
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, FromRow)]
pub struct Token {
    pub user_id: uuid::Uuid,
    pub token: String,
    pub expired_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Iden)]
pub enum Tokens {
    Table,
    UserId,
    Token,
    ExpiredAt,
}
