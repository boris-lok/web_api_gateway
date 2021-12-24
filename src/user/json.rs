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

impl From<User> for SimpleUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id.unwrap_or_else(uuid::Uuid::new_v4),
            name: user.name,
            role: user.role,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Debug, Clone)]
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
