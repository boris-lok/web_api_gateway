use common::json::customer::Customer;
use warp::reject::Rejection;
use warp::reply::{Reply, Response};

use crate::customer::json::{CreateCustomerRequest, ListCustomerRequest, UpdateCustomerRequest};
use crate::pb::GetCustomerRequest;
use crate::Env;

pub async fn get(req: u64, env: Env) -> Result<Response, Rejection> {
    let mut client = env.grpc_customer_client;

    client
        .get(GetCustomerRequest { id: req })
        .await
        .map(|c| {
            let customer = c.into_inner().customer;
            match customer {
                None => warp::reply::reply().into_response(),
                Some(c) => {
                    let c: Customer = c.into();
                    warp::reply::json(&c).into_response()
                }
            }
        })
        .map_err(|err| warp::reject::reject())
}

pub async fn create(req: CreateCustomerRequest, env: Env) -> Result<impl Reply, Rejection> {
    let mut client = env.grpc_customer_client;

    let req: crate::pb::CreateCustomerRequest = req.into();

    client
        .create(req)
        .await
        .map(|c| {
            let c: Customer = c.into_inner().into();
            warp::reply::json(&c)
        })
        .map_err(|err| {
            let msg = err.to_string();
            tracing::error!(%msg);

            warp::reject::reject()
        })
}

pub async fn update(req: UpdateCustomerRequest, env: Env) -> Result<impl Reply, Rejection> {
    let mut client = env.grpc_customer_client;

    let req: crate::pb::UpdateCustomerRequest = req.into();

    client
        .update(req)
        .await
        .map(|c| {
            let c: Customer = c.into_inner().into();
            warp::reply::json(&c)
        })
        .map_err(|err| warp::reject::reject())
}

pub async fn list(req: ListCustomerRequest, env: Env) -> Result<impl Reply, Rejection> {
    let mut client = env.grpc_customer_client;

    let req: crate::pb::ListCustomerRequest = req.into();

    client
        .list(req)
        .await
        .map(|c| {
            let c = c
                .into_inner()
                .customers
                .into_iter()
                .map(|c| c.into())
                .collect::<Vec<Customer>>();
            warp::reply::json(&c)
        })
        .map_err(|err| warp::reject::reject())
}
