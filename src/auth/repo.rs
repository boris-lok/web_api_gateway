use std::sync::Arc;

use async_trait::async_trait;
use mockall::predicate::*;
use mockall::*;
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::auth::json::{Token, Tokens};
use crate::core::error::AppError;

#[automock]
#[async_trait]
pub trait AuthRepository {
    async fn create(
        &self,
        id: uuid::Uuid,
        token: &str,
        expired_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<Token, AppError>;

    async fn expire(&self, id: uuid::Uuid) -> Result<(), AppError>;

    async fn renew(
        &self,
        id: uuid::Uuid,
        expired_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), AppError>;
}

#[derive(Clone)]
pub struct PostgresAuthRepository {
    connection_pool: Arc<Pool<Postgres>>,
}

impl PostgresAuthRepository {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self {
            connection_pool: pool,
        }
    }

    async fn update_expired_at(
        &self,
        id: Uuid,
        expired_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), AppError> {
        let sql = Query::update()
            .table(Tokens::Table)
            .values(vec![(Tokens::ExpiredAt, expired_at.into())])
            .and_where(Expr::col(Tokens::UserId).eq(id.to_string()))
            .to_string(PostgresQueryBuilder);

        dbg!(&sql);

        let res = sqlx::query(sql.as_str())
            .execute(&*self.connection_pool)
            .await
            .map(|res| res.rows_affected())
            .map_err(AppError::DatabaseError);

        dbg!(&res);

        Ok(())
    }
}

#[async_trait]
impl AuthRepository for PostgresAuthRepository {
    async fn create(
        &self,
        id: Uuid,
        token: &str,
        expired_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<Token, AppError> {
        let sql = Query::insert()
            .into_table(Tokens::Table)
            .columns(vec![Tokens::UserId, Tokens::Token, Tokens::ExpiredAt])
            .values_panic(vec![id.into(), token.into(), expired_at.into()])
            .returning(
                Query::select()
                    .columns(vec![Tokens::UserId, Tokens::Token, Tokens::ExpiredAt])
                    .take(),
            )
            .to_string(PostgresQueryBuilder);

        dbg!(&sql);

        let token = sqlx::query_as::<_, Token>(sql.as_str())
            .fetch_one(&*self.connection_pool)
            .await
            .map_err(AppError::DatabaseError);

        dbg!(&token);

        token
    }

    async fn expire(&self, id: Uuid) -> Result<(), AppError> {
        let expired_at = chrono::Utc::now();

        self.update_expired_at(id, expired_at).await
    }

    async fn renew(
        &self,
        id: Uuid,
        expired_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), AppError> {
        self.update_expired_at(id, expired_at).await
    }
}
