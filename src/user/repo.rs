use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::core::error::AppError;
use crate::user::json::{SimpleUser, User, Users};

#[async_trait]
pub trait UserRepository {
    async fn create(&mut self, user: &User) -> Result<SimpleUser, AppError>;

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
pub struct PostgresRepository {
    connection_pool: Arc<Pool<Postgres>>,
}

impl PostgresRepository {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self {
            connection_pool: pool,
        }
    }
}

#[async_trait]
impl UserRepository for PostgresRepository {
    async fn create(&mut self, user: &User) -> Result<SimpleUser, AppError> {
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
            .await;

        dbg!(&simple_user);

        simple_user.map_err(|_| AppError::DatabaseError)
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
            .await;

        dbg!(&simple_user);

        simple_user.map_err(|_| AppError::DatabaseError)
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
            .await;

        dbg!(&simple_users);

        simple_users.map_err(|_| AppError::DatabaseError)
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
            .await;

        dbg!(&simple_user);

        simple_user.map_err(|_| AppError::DatabaseError)
    }
}

pub struct MockUserRepository {
    data: HashMap<uuid::Uuid, User>,
}

impl MockUserRepository {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

#[async_trait]
impl UserRepository for MockUserRepository {
    async fn create(&mut self, user: &User) -> Result<SimpleUser, AppError> {
        let id = uuid::Uuid::new_v4();
        let simple_user = SimpleUser {
            id,
            name: user.name.clone(),
            role: 0,
            created_at: user.created_at,
            updated_at: user.updated_at,
        };
        self.data.insert(id, user.clone());
        Ok(simple_user)
    }

    async fn get(&self, id: &Uuid) -> Result<Option<SimpleUser>, AppError> {
        let user = self.data.get(id);
        Ok(user.map(|user| user.clone().into()))
    }

    async fn list(
        &self,
        keyword: Option<String>,
        updated_at: Option<chrono::DateTime<chrono::Utc>>,
        page_size: usize,
    ) -> Result<Vec<SimpleUser>, AppError> {
        let capacity = if self.data.len() < page_size {
            self.data.len()
        } else {
            page_size
        };
        let mut result = Vec::with_capacity(capacity);
        let keys: Vec<_> = self.data.iter().map(|x| *x.0).collect();
        let count = &self.data.len();
        for i in 0..*count {
            let key = keys.get(i).unwrap();
            let data = self.data.get(key).unwrap();
            if updated_at.unwrap() < data.updated_at.unwrap() {
                if keyword.is_none() {
                    result.push(data.clone());
                } else {
                    let keyword = keyword.clone().unwrap();
                    if data.name.contains(&keyword) {
                        result.push(data.clone());
                    }
                }
            }
        }

        let s1: Vec<_> = result
            .iter()
            .take(page_size)
            .map(|x| x.clone().into())
            .collect();

        Ok(s1)
    }

    async fn check_user_is_exist(
        &self,
        username: &str,
        password: &str,
    ) -> Result<Option<SimpleUser>, AppError> {
        for v in self.data.values() {
            if v.name == username && v.password == password {
                return Ok(Some(v.clone().into()));
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use chrono::Utc;

    use super::*;

    impl User {
        pub fn mock(name: &str) -> Self {
            Self {
                id: None,
                name: name.to_string(),
                password: "password".to_string(),
                role: 0,
                created_at: Some(chrono::Utc::now()),
                updated_at: Some(chrono::Utc::now()),
            }
        }
    }

    #[test]
    fn it_can_create_a_user() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let mut repo = MockUserRepository::new();
        let user = User::mock("boris");
        let predict_user = runtime.block_on(repo.create(&user));
        assert!(predict_user.is_ok());
        let predict_user = predict_user.ok().unwrap();
        assert_eq!(predict_user.name, "boris".to_string());
        assert_eq!(predict_user.role, 0);
    }

    #[test]
    fn it_can_get_a_user() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let mut repo = MockUserRepository::new();
        let user = User::mock("boris");
        let result = runtime.block_on(repo.create(&user));
        let predict_user = runtime.block_on(repo.get(&result.ok().unwrap().id));
        assert!(predict_user.is_ok());
        let predict_user = predict_user.ok().flatten().unwrap();
        assert_eq!(predict_user.name, "boris".to_string());
        assert_eq!(predict_user.role, 0);
    }

    #[test]
    fn it_can_list_users() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let mut repo = MockUserRepository::new();
        for i in 0..100 {
            let user = User::mock(format!("user_{}", i).as_str());
            let result = runtime.block_on(repo.create(&user));
        }

        let predict_users = runtime.block_on(repo.list(
            None,
            Some(chrono::Utc::now() - chrono::Duration::days(1)),
            30,
        ));
        assert!(predict_users.is_ok());
        let predict_users = predict_users.ok().unwrap();
        assert_eq!(predict_users.len(), 30);

        let predict_users = runtime.block_on(repo.list(
            Some("1".to_string()),
            Some(chrono::Utc::now() - chrono::Duration::days(1)),
            30,
        ));
        assert!(predict_users.is_ok());
        let predict_users = predict_users.ok().unwrap();
        assert_eq!(predict_users.len(), 19);
    }

    #[test]
    fn it_can_find_a_user() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let mut repo = MockUserRepository::new();
        let user = User::mock("boris");
        let result = runtime.block_on(repo.create(&user));
        let predict_user = runtime.block_on(repo.check_user_is_exist("boris"));
        assert!(predict_user.is_ok());
        let predict_user = predict_user.ok().flatten();
        assert!(predict_user.is_some());
    }

    #[test]
    fn it_can_find_not_a_user() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let mut repo = MockUserRepository::new();
        let user = User::mock("boris");
        let result = runtime.block_on(repo.create(&user));
        let predict_user = runtime.block_on(repo.check_user_is_exist("boris1"));
        assert!(predict_user.is_ok());
        let predict_user = predict_user.ok().flatten();
        assert!(predict_user.is_none());
    }
}
