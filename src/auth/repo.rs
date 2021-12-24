use crate::auth::json::{Token, Tokens};
use async_trait::async_trait;
use mockall::predicate::*;
use mockall::*;
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sqlx::{Pool, Postgres};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use crate::core::error::AppError;

#[automock]
#[async_trait]
pub trait AuthRepository {
    async fn create(&self, id: uuid::Uuid, token: &str, after_days: u8) -> Result<Token, AppError>;

    async fn expire(&self, id: uuid::Uuid) -> Result<(), AppError>;

    async fn renew(&self, id: uuid::Uuid, after_days: u8) -> Result<(), AppError>;
}

struct PostgresAuthRepository {
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
            .values(vec![(Tokens::ExpiredAt, expired_at.to_string().into())])
            .and_where(Expr::col(Tokens::UserId).eq(id.to_string()))
            .to_string(PostgresQueryBuilder);

        dbg!(&sql);

        let res = sqlx::query(sql.as_str())
            .execute(&*self.connection_pool)
            .await
            .map(|res| res.rows_affected())
            .map_err(|e| AppError::DatabaseError(e));

        dbg!(&res);

        Ok(())
    }
}

#[async_trait]
impl AuthRepository for PostgresAuthRepository {
    async fn create(&self, id: Uuid, token: &str, after_days: u8) -> Result<Token, AppError> {
        let expired_at = chrono::Utc::now() + chrono::Duration::days(after_days as i64);

        let sql = Query::insert()
            .into_table(Tokens::Table)
            .columns(vec![Tokens::UserId, Tokens::Token, Tokens::ExpiredAt])
            .values_panic(vec![
                id.to_string().into(),
                token.into(),
                expired_at.to_string().into(),
            ])
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
            .map_err(|e| AppError::DatabaseError(e));

        dbg!(&token);

        token
    }

    async fn expire(&self, id: Uuid) -> Result<(), AppError> {
        let expired_at = chrono::Utc::now();

        self.update_expired_at(id, expired_at).await
    }

    async fn renew(&self, id: Uuid, after_days: u8) -> Result<(), AppError> {
        let expired_at = chrono::Utc::now() + chrono::Duration::days(after_days as i64);

        self.update_expired_at(id, expired_at).await
    }
}
