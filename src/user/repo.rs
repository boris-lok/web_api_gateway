use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::core::error::AppError;
use crate::user::json::table::Users;
use crate::user::json::user::{SimpleUser, User};

#[async_trait]
pub trait UserRepository {
    async fn create(
        &self,
        username: &str,
        password: &str,
        role: i16,
    ) -> Result<SimpleUser, AppError>;

    async fn get(&self, id: &uuid::Uuid) -> Result<Option<SimpleUser>, AppError>;

    async fn list(
        &self,
        keyword: Option<String>,
        updated_at: Option<chrono::DateTime<chrono::Utc>>,
        page_size: usize,
    ) -> Result<Vec<SimpleUser>, AppError>;

    async fn get_by_name(&self, username: &str) -> Result<Option<User>, AppError>;
}

#[derive(Clone)]
pub struct PostgresUserRepository {
    connection_pool: Arc<Pool<Postgres>>,
}

impl PostgresUserRepository {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self {
            connection_pool: pool,
        }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(
        &self,
        username: &str,
        password: &str,
        role: i16,
    ) -> Result<SimpleUser, AppError> {
        let sql = Query::insert()
            .into_table(Users::Table)
            .columns(vec![
                Users::Id,
                Users::Name,
                Users::Password,
                Users::Role,
                Users::CreatedAt,
                Users::UpdatedAt,
            ])
            .values_panic(vec![
                uuid::Uuid::new_v4().into(),
                username.into(),
                password.into(),
                role.into(),
                chrono::Utc::now().into(),
                chrono::Utc::now().into(),
            ])
            .to_owned()
            .returning(
                Query::select()
                    .columns(vec![
                        Users::Id,
                        Users::Name,
                        Users::Role,
                        Users::CreatedAt,
                        Users::UpdatedAt,
                    ])
                    .take(),
            )
            .to_string(PostgresQueryBuilder);

        dbg!(&sql);

        let simple_user = sqlx::query_as::<_, SimpleUser>(sql.as_str())
            .fetch_one(&*self.connection_pool)
            .await
            .map_err(|_| AppError::DatabaseError);

        dbg!(&simple_user);

        simple_user
    }

    async fn get(&self, id: &Uuid) -> Result<Option<SimpleUser>, AppError> {
        let sql = Query::select()
            .columns(vec![
                Users::Id,
                Users::Name,
                Users::Role,
                Users::CreatedAt,
                Users::UpdatedAt,
            ])
            .from(Users::Table)
            .and_where(Expr::col(Users::Id).eq(id.to_string()))
            .to_string(PostgresQueryBuilder);

        dbg!(&sql);

        let simple_user = sqlx::query_as::<_, SimpleUser>(sql.as_str())
            .fetch_optional(&*self.connection_pool)
            .await
            .map_err(|_| AppError::DatabaseError);

        dbg!(&simple_user);

        simple_user
    }

    async fn list(
        &self,
        keyword: Option<String>,
        updated_at: Option<DateTime<Utc>>,
        page_size: usize,
    ) -> Result<Vec<SimpleUser>, AppError> {
        let sql = Query::select()
            .columns(vec![
                Users::Id,
                Users::Name,
                Users::Role,
                Users::CreatedAt,
                Users::UpdatedAt,
            ])
            .and_where_option(
                keyword.map(|e| Expr::col(Users::Name).like(format!("%{}%", e).as_str())),
            )
            .and_where_option(updated_at.map(|e| Expr::col(Users::UpdatedAt).gt(e)))
            .from(Users::Table)
            .limit(page_size as u64)
            .to_string(PostgresQueryBuilder);

        dbg!(&sql);

        let simple_users = sqlx::query_as::<_, SimpleUser>(sql.as_str())
            .fetch_all(&*self.connection_pool)
            .await
            .map_err(|_| AppError::DatabaseError);

        dbg!(&simple_users);

        simple_users
    }

    async fn get_by_name(&self, username: &str) -> Result<Option<User>, AppError> {
        let sql = Query::select()
            .columns(vec![
                Users::Id,
                Users::Name,
                Users::Password,
                Users::Role,
                Users::CreatedAt,
                Users::UpdatedAt,
            ])
            .from(Users::Table)
            .and_where(Expr::col(Users::Name).eq(username))
            .to_string(PostgresQueryBuilder);

        dbg!(&sql);

        let user = sqlx::query_as::<_, User>(sql.as_str())
            .fetch_optional(&*self.connection_pool)
            .await
            .map_err(|_| AppError::DatabaseError);

        dbg!(&user);

        user
    }
}
