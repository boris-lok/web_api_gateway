#![allow(unused_variables, dead_code)]

use std::sync::Arc;

use r2d2_redis::{r2d2, RedisConnectionManager};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::Postgres;
use warp::Filter;

use crate::auth::repo::RedisAuthRepository;
use crate::core::config::Config;
use crate::core::environment::Environment;
use crate::core::recover::rejection_handler;
use crate::user::repo::PostgresUserRepository;

mod auth;
mod core;
mod user;

type AppResult<T> = anyhow::Result<T>;
type WebResult<T> = std::result::Result<T, warp::reject::Rejection>;

#[tokio::main]
async fn main() {
    let config = Config::new();

    tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .init();

    let database_connection_pool = create_database_connection(&config)
        .await
        .expect("Can't create a database connection pool.");

    let redis_connection_pool =
        create_redis_connection(&config).expect("Can't create a redis connection pool");

    let connection_pool = Arc::new(database_connection_pool);
    let user_repo = PostgresUserRepository::new(Arc::clone(&connection_pool));
    let auth_repo = RedisAuthRepository::new(redis_connection_pool);

    let env = Environment::new(config, Arc::new(auth_repo), Arc::new(user_repo));

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "access-control-allow-origin",
            "content-type"
        ])
        .allow_credentials(true)
        .expose_headers(vec!["set-cookie"])
        .allow_methods(vec!["GET", "POST", "DELETE", "PUT", "PATCH"]);

    let auth_routes = auth::route::routes(env.clone());
    let user_routes = user::route::routes(env.clone());
    let routes = auth_routes
        .or(user_routes)
        .with(cors)
        .recover(rejection_handler)
        .with(warp::trace::request());

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    connection_pool.close().await;
}

async fn create_database_connection(config: &Config) -> Option<sqlx::Pool<Postgres>> {
    let connection_options = PgConnectOptions::new()
        .host(&config.postgres_host)
        .database(&config.postgres_database)
        .username(&config.postgres_username)
        .password(&config.postgres_password)
        .port(config.postgres_port);

    let connection_pool = PgPoolOptions::new()
        .max_connections(config.postgres_max_connections)
        .connect_with(connection_options)
        .await
        .expect("Can't connect to database");

    Some(connection_pool)
}

fn create_redis_connection(config: &Config) -> Option<r2d2::Pool<RedisConnectionManager>> {
    let redis_uri = format!(
        "redis://{}:{}@{}:{}",
        &config.redis_username.as_ref().unwrap_or(&"".to_string()),
        &config.redis_password,
        &config.redis_host,
        config.redis_port
    );

    RedisConnectionManager::new(redis_uri)
        .map(|manager| r2d2::Pool::builder().build(manager).ok())
        .ok()
        .flatten()
}
