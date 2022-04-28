#![allow(unused_variables, dead_code)]

use std::sync::Arc;

use r2d2_redis::{r2d2, RedisConnectionManager};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::Postgres;
use warp::Filter;

use common::configs::postgres_config::PostgresConfig;
use common::configs::redis_config::RedisConfig;
use common::utils::tools::{create_database_connection, create_redis_connection};

use crate::utils::env::Env;
use pb::customer_services_client::CustomerServicesClient;

mod customer;
mod utils;

mod pb {
    include!("../gen/grpc.customer.rs");
}

#[tokio::main]
async fn main() {
    dotenv::from_path("env/dev.env");

    tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .init();

    let postgres = PostgresConfig::new();
    let database_connection_pool = create_database_connection(postgres)
        .await
        .expect("Can create a database connection pool.");

    // let redis = RedisConfig::new();
    // let redis_connection = create_redis_connection(redis)
    //    .await
    //    .expect("Can create a redis connection.");

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Access-Control-Allow-Origin", "Content-Type"])
        .allow_credentials(true)
        .expose_headers(vec!["set-cookie"])
        .allow_methods(vec!["GET", "POST", "DELETE", "PUT", "PATCH"]);

    let grpc_customer_client = CustomerServicesClient::connect("http://127.0.0.1:50001")
        .await
        .unwrap();

    let env = Env::new(true, grpc_customer_client);

    // let routes = proxy_routes
    //     .with(cors)
    //     .with(warp::trace::request());
    // .recover(rejection_handler);

    let routes = customer::routes::routes(env.clone())
        .with(cors)
        .with(warp::trace::request());

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    database_connection_pool.close().await;
}
