use crate::user::repo::PostgresRepository;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Pool, Postgres};
use std::sync::Arc;

mod auth;
mod core;
mod user;

#[tokio::main]
async fn main() {
    let debug = dotenv::var("DEBUG")
        .map(|x| x.parse::<bool>().unwrap_or_else(|_| true))
        .unwrap_or_else(|_| true);

    if debug {
        let _ = dotenv::from_path("dev.env").expect("Can't find the dev.env.");
    } else {
        let _ = dotenv::from_path("prod.env").expect("Can't find the prod.env.");
    }

    let connection_pool = create_database_connection()
        .await
        .expect("Can't create a database connection pool.");

    let connection_pool = Arc::new(connection_pool);
    let repo = PostgresRepository::new(Arc::clone(&connection_pool));

    connection_pool.close().await;
}

async fn create_database_connection() -> Option<Pool<Postgres>> {
    let postgres_host = dotenv::var("POSTGRES_HOST").expect("Can't read postgres_host from env.");
    let postgres_user = dotenv::var("POSTGRES_USER").expect("Can't read postgres_user from env.");
    let postgres_password =
        dotenv::var("POSTGRES_PASSWORD").expect("Can't read postgres_password from env.");
    let postgres_port = dotenv::var("POSTGRES_PORT")
        .expect("Can't read postgres_port from env.")
        .parse::<u16>()
        .expect("Can't parse the port to u16 type.");
    let postgres_database = dotenv::var("POSTGRES_DB").expect("Can't read postgres_db from env.");

    let connection_options = PgConnectOptions::new()
        .host(&postgres_host)
        .database(&postgres_database)
        .username(&postgres_user)
        .password(&postgres_password)
        .port(postgres_port);

    let connection_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(connection_options)
        .await
        .expect("Can't connect to database");

    Some(connection_pool)
}
