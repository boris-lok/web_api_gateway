use sea_query::Iden;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct SimpleUser {
    pub id: uuid::Uuid,
    pub name: String,
    pub role: i16,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug)]
pub struct User {
    pub id: Option<uuid::Uuid>,
    pub name: String,
    pub password: String,
    pub role: i16,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Iden)]
pub enum Users {
    Table,
    Id,
    Name,
    Password,
    Role,
    CreatedAt,
    UpdatedAt,
}
