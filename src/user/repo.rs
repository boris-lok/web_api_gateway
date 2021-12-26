use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use mockall::predicate::*;
use mockall::*;
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::core::error::AppError;
use crate::user::json::{SimpleUser, User, Users};

#[automock]
#[async_trait]
pub trait UserRepository {
    async fn create(&self, user: &User) -> Result<SimpleUser, AppError>;

    async fn get(&self, id: &uuid::Uuid) -> Result<Option<SimpleUser>, AppError>;

    async fn list(
        &self,
        keyword: Option<String>,
        updated_at: Option<chrono::DateTime<chrono::Utc>>,
        page_size: usize,
    ) -> Result<Vec<SimpleUser>, AppError>;

    async fn check_user_is_exist(
        &self,
        username: &str,
        password: &str,
    ) -> Result<Option<SimpleUser>, AppError>;
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
    async fn create(&self, user: &User) -> Result<SimpleUser, AppError> {
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
                uuid::Uuid::new_v4().to_string().into(),
                user.name.as_str().into(),
                user.password.as_str().into(),
                user.role.into(),
                chrono::Utc::now().to_string().into(),
                chrono::Utc::now().to_string().into(),
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
            .map_err(AppError::DatabaseError);

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
            .and_where(Expr::col(Users::Id).eq(id.to_string().as_str()))
            .to_string(PostgresQueryBuilder);

        dbg!(&sql);

        let simple_user = sqlx::query_as::<_, SimpleUser>(sql.as_str())
            .fetch_optional(&*self.connection_pool)
            .await
            .map_err(AppError::DatabaseError);

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
            .and_where_option(
                updated_at.map(|e| Expr::col(Users::UpdatedAt).gt(e.to_string().as_str())),
            )
            .from(Users::Table)
            .limit(page_size as u64)
            .to_string(PostgresQueryBuilder);

        dbg!(&sql);

        let simple_users = sqlx::query_as::<_, SimpleUser>(sql.as_str())
            .fetch_all(&*self.connection_pool)
            .await
            .map_err(AppError::DatabaseError);

        dbg!(&simple_users);

        simple_users
    }

    async fn check_user_is_exist(
        &self,
        username: &str,
        password: &str,
    ) -> Result<Option<SimpleUser>, AppError> {
        let sql = Query::select()
            .columns(vec![
                Users::Id,
                Users::Name,
                Users::Role,
                Users::CreatedAt,
                Users::UpdatedAt,
            ])
            .from(Users::Table)
            .and_where(Expr::col(Users::Name).eq(username))
            .and_where(Expr::col(Users::Password).eq(password))
            .to_string(PostgresQueryBuilder);

        dbg!(&sql);

        let simple_user = sqlx::query_as::<_, SimpleUser>(sql.as_str())
            .fetch_optional(&*self.connection_pool)
            .await
            .map_err(AppError::DatabaseError);

        dbg!(&simple_user);

        simple_user
    }
}
