use std::sync::Arc;

use crate::{Config, RedisAuthRepository, PostgresUserRepository};

#[derive(Clone)]
pub struct Environment {
    pub config: Config,
    pub auth_repo: Arc<RedisAuthRepository>,
    pub user_repo: Arc<PostgresUserRepository>,
}

impl Environment {
    pub fn new(
        config: Config,
        auth_repo: Arc<RedisAuthRepository>,
        user_repo: Arc<PostgresUserRepository>,
    ) -> Self {
        Self {
            config,
            auth_repo,
            user_repo,
        }
    }
}
