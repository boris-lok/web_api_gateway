#![allow(unused_variables, dead_code)]

use std::sync::Arc;

use crate::core::config::Config;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Pool, Postgres};

use crate::user::repo::PostgresUserRepository;

mod auth;
mod core;
mod user;

#[tokio::main]
async fn main() {
    let config = Config::new();

    let connection_pool = create_database_connection(&config)
        .await
        .expect("Can't create a database connection pool.");

    let connection_pool = Arc::new(connection_pool);
    let repo = PostgresUserRepository::new(Arc::clone(&connection_pool));

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
