#![allow(unused_variables, dead_code)]

use std::sync::Arc;

use sqlx::{Pool, Postgres};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use warp::Filter;

use crate::auth::repo::PostgresAuthRepository;
use crate::core::config::Config;
use crate::core::environment::Environment;
use crate::user::repo::PostgresUserRepository;

mod auth;
mod core;
mod user;

type WebResult<T> = std::result::Result<T, warp::reject::Rejection>;

#[tokio::main]
async fn main() {
    let config = Config::new();

    tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .init();

    let connection_pool = create_database_connection(&config)
        .await
        .expect("Can't create a database connection pool.");

    let connection_pool = Arc::new(connection_pool);
    let user_repo = PostgresUserRepository::new(Arc::clone(&connection_pool));
    let auth_repo = PostgresAuthRepository::new(Arc::clone(&connection_pool));

    let env = Environment::new(config, Arc::new(auth_repo), Arc::new(user_repo));

    let auth_routes = auth::route::routes(env.clone());
    let user_routes = user::route::routes(env.clone());
    let routes = auth_routes.or(user_routes).with(warp::trace::request());

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    connection_pool.close().await;
}

async fn create_database_connection(config: &Config) -> Option<Pool<Postgres>> {
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
