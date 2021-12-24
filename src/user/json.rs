use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct SimpleUser {
    pub id: uuid::Uuid,
    pub name: String,
    pub role: String,
}

pub struct User {
    pub id: Option<uuid::Uuid>,
    pub name: String,
    pub password: String,
    pub role: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
