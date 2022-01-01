use sea_query::Iden;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Token {
    pub user_id: uuid::Uuid,
    pub token: String,
    pub expired_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
    role: u8,
}

impl Claims {
    pub fn new(sub: String, exp: usize, role: u8) -> Self {
        Self { sub, exp, role }
    }
}

#[derive(Iden)]
pub enum Tokens {
    Table,
    UserId,
    Token,
    ExpiredAt,
}
