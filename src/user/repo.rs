use std::borrow::Borrow;
use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::core::error::AppError;
use crate::user::json::{SimpleUser, User};

#[async_trait]
trait UserRepository {
    async fn create(&mut self, user: &User) -> Result<SimpleUser, AppError>;

    async fn get(&self, id: &uuid::Uuid) -> Result<Option<SimpleUser>, AppError>;

    async fn list(
        &self,
        keyword: Option<String>,
        offset: usize,
        page_size: usize,
    ) -> Result<Vec<SimpleUser>, AppError>;

    async fn find(&self, username: &str) -> Result<Option<SimpleUser>, AppError>;
}

#[derive(Clone)]
pub struct PostgresRepository {
    connection_pool: Arc<Pool<Postgres>>,
}

impl PostgresRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {
            connection_pool: Arc::new(pool),
        }
    }
}

pub struct MockUserRepository {
    data: HashMap<uuid::Uuid, SimpleUser>,
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
            role: user.role.clone(),
        };
        self.data.insert(id.clone(), simple_user.clone());
        Ok(simple_user)
    }

    async fn get(&self, id: &Uuid) -> Result<Option<SimpleUser>, AppError> {
        let user = self.data.get(id);
        Ok(user.cloned())
    }

    async fn list(
        &self,
        keyword: Option<String>,
        offset: usize,
        page_size: usize,
    ) -> Result<Vec<SimpleUser>, AppError> {
        let capacity = if self.data.len() < page_size {
            self.data.len()
        } else {
            page_size
        };
        let mut result = Vec::with_capacity(capacity);
        let keys: Vec<_> = self.data.iter().map(|x| x.0.clone()).collect();
        let count = &self.data.len();
        for i in 0..*count {
            let key = keys.get(i).unwrap();
            let data = self.data.get(key).unwrap();
            if keyword.is_none() {
                result.push(data.clone());
            } else {
                let keyword = keyword.clone().unwrap();
                if data.name.contains(&keyword) {
                    result.push(data.clone());
                }
            }
        }

        let s1: Vec<_> = result
            .iter()
            .skip(offset)
            .take(page_size)
            .map(|x| x.clone())
            .collect();

        Ok(s1)
    }

    async fn find(&self, username: &str) -> Result<Option<SimpleUser>, AppError> {
        for (_, v) in &self.data {
            if v.name == username {
                return Ok(Some(v.clone()));
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_create_a_user() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let mut repo = MockUserRepository::new();
        let user = User {
            id: None,
            name: "boris".to_string(),
            password: "password".to_string(),
            role: "admin".to_string(),
            created_at: None,
            updated_at: None,
        };
        let predict_user = runtime.block_on(repo.create(&user));
        assert!(predict_user.is_ok());
        let predict_user = predict_user.ok().unwrap();
        assert_eq!(predict_user.name, "boris".to_string());
        assert_eq!(predict_user.role, "admin".to_string());
    }

    #[test]
    fn it_can_get_a_user() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let mut repo = MockUserRepository::new();
        let user = User {
            id: None,
            name: "boris".to_string(),
            password: "password".to_string(),
            role: "admin".to_string(),
            created_at: None,
            updated_at: None,
        };
        let result = runtime.block_on(repo.create(&user));
        let predict_user = runtime.block_on(repo.get(&result.ok().unwrap().id));
        assert!(predict_user.is_ok());
        let predict_user = predict_user.ok().flatten().unwrap();
        assert_eq!(predict_user.name, "boris".to_string());
        assert_eq!(predict_user.role, "admin".to_string());
    }

    #[test]
    fn it_can_list_users() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let mut repo = MockUserRepository::new();
        for i in 0..100 {
            let user = User {
                id: None,
                name: format!("user_{}", i),
                password: "password".to_string(),
                role: "admin".to_string(),
                created_at: None,
                updated_at: None,
            };
            let result = runtime.block_on(repo.create(&user));
        }

        let predict_users = runtime.block_on(repo.list(None, 0, 30));
        assert!(predict_users.is_ok());
        let predict_users = predict_users.ok().unwrap();
        assert_eq!(predict_users.len(), 30);

        let predict_users = runtime.block_on(repo.list(Some("1".to_string()), 0, 30));
        assert!(predict_users.is_ok());
        let predict_users = predict_users.ok().unwrap();
        assert_eq!(predict_users.len(), 19);

        let predict_users = runtime.block_on(repo.list(Some("1".to_string()), 10, 30));
        assert!(predict_users.is_ok());
        let predict_users = predict_users.ok().unwrap();
        assert_eq!(predict_users.len(), 9);
    }

    #[test]
    fn it_can_find_a_user() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let mut repo = MockUserRepository::new();
        let user = User {
            id: None,
            name: "boris".to_string(),
            password: "password".to_string(),
            role: "admin".to_string(),
            created_at: None,
            updated_at: None,
        };
        let result = runtime.block_on(repo.create(&user));
        let predict_user = runtime.block_on(repo.find("boris"));
        assert!(predict_user.is_ok());
        let predict_user = predict_user.ok().flatten();
        assert!(predict_user.is_some());
    }

    #[test]
    fn it_can_find_not_a_user() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let mut repo = MockUserRepository::new();
        let user = User {
            id: None,
            name: "boris".to_string(),
            password: "password".to_string(),
            role: "admin".to_string(),
            created_at: None,
            updated_at: None,
        };
        let result = runtime.block_on(repo.create(&user));
        let predict_user = runtime.block_on(repo.find("boris1"));
        assert!(predict_user.is_ok());
        let predict_user = predict_user.ok().flatten();
        assert!(predict_user.is_none());
    }
}
