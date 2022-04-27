use async_trait::async_trait;
use r2d2_redis::{r2d2, redis, RedisConnectionManager};
use uuid::Uuid;

use crate::AppResult;

pub trait AuthRepository {
    fn create(&self, id: Uuid, token: &str, seconds: usize) -> AppResult<String>;

    fn expire(&self, id: Uuid) -> AppResult<()>;

    fn renew(&self, id: Uuid, seconds: usize) -> AppResult<()>;

    fn get(&self, id: Uuid) -> Option<String>;
}

#[derive(Clone)]
pub struct RedisAuthRepository {
    connection_pool: r2d2::Pool<RedisConnectionManager>,
}

impl RedisAuthRepository {
    pub fn new(pool: r2d2::Pool<RedisConnectionManager>) -> Self {
        Self {
            connection_pool: pool,
        }
    }

    fn user_id_to_key(&self, id: &Uuid) -> String {
        format!("user_id: {}", id)
    }

    fn get_connection(&self) -> r2d2::PooledConnection<RedisConnectionManager> {
        self.connection_pool.get().expect("Can't get redis pool")
    }
}

#[async_trait]
impl AuthRepository for RedisAuthRepository {
    fn create(&self, id: Uuid, token: &str, seconds: usize) -> AppResult<String> {
        redis::cmd("SETEX")
            .arg(self.user_id_to_key(&id))
            .arg(seconds)
            .arg(token)
            .execute(&mut *self.get_connection());

        Ok(token.to_owned())
    }

    fn expire(&self, id: Uuid) -> AppResult<()> {
        redis::cmd("DEL")
            .arg(self.user_id_to_key(&id))
            .execute(&mut *self.get_connection());

        Ok(())
    }

    fn renew(&self, id: Uuid, seconds: usize) -> AppResult<()> {
        redis::cmd("Expire")
            .arg(self.user_id_to_key(&id))
            .arg(seconds)
            .execute(&mut *self.get_connection());

        Ok(())
    }

    fn get(&self, id: Uuid) -> Option<String> {
        redis::cmd("GET")
            .arg(self.user_id_to_key(&id))
            .query::<String>(&mut *self.get_connection())
            .ok()
    }
}
