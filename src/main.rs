#![allow(unused_variables, dead_code)]

use warp::Filter;

use common::configs::config::Config;
use common::configs::postgres_config::PostgresConfig;
use common::utils::tools::{create_database_connection, tracing_initialize};
use pb::customer_services_client::CustomerServicesClient;
use pb::product_services_client::ProductServicesClient;

use crate::utils::env::Env;
use crate::utils::recover::rejection_handler;

mod customer;
mod product;
mod utils;

mod pb {
    include!("../gen/grpc.customer.rs");
    include!("../gen/grpc.product.rs");
}

#[tokio::main]
async fn main() {
    let _ = dotenv::from_path("env/dev.env").unwrap();

    let config = Config::new();
    tracing_initialize(config.debug, "logs/", "gateway");

    let postgres = PostgresConfig::new();
    let database_connection_pool = create_database_connection(postgres)
        .await
        .expect("Can create a database connection pool.");

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Access-Control-Allow-Origin", "Content-Type"])
        .allow_credentials(true)
        .expose_headers(vec!["set-cookie"])
        .allow_methods(vec!["GET", "POST", "DELETE", "PUT", "PATCH"]);

    let grpc_customer_client = CustomerServicesClient::connect("http://127.0.0.1:50001")
        .await
        .unwrap();

    let grpc_product_client = ProductServicesClient::connect("http://127.0.0.1:50002").await.unwrap();

    let env = Env::new(true, grpc_customer_client, grpc_product_client);

    let routes = customer::routes::routes(env.clone())
        .with(cors)
        .with(warp::trace::request())
        .recover(rejection_handler);

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;

    database_connection_pool.close().await;
}
