use warp::hyper::StatusCode;

use crate::customer::json::{CreateCustomerRequest, Customer};
use crate::pb::GetCustomerRequest;
use crate::Env;

pub async fn get(req: u64, env: Env) -> Result<Box<dyn warp::Reply>, warp::reject::Rejection> {
    let mut client = env.grpc_customer_client;

    let response = client.get(GetCustomerRequest { id: req }).await;

    if response.is_err() {
        return Err(warp::reject::reject());
    }

    let customer = response.unwrap().into_inner().customer;

    if customer.is_none() {
        return Ok(Box::new(StatusCode::OK));
    }

    let customer: Customer = customer.unwrap().into();

    Ok(Box::new(warp::reply::json(&customer)))
}

pub async fn create(
    req: CreateCustomerRequest,
    env: Env,
) -> Result<Box<dyn warp::Reply>, warp::reject::Rejection> {
    let mut client = env.grpc_customer_client;

    let req: crate::pb::CreateCustomerRequest = req.into();

    let response = client.create(req).await;

    if response.is_err() {
        return Err(warp::reject::reject());
    }

    let customer: Customer = response.unwrap().into_inner().into();

    Ok(Box::new(warp::reply::json(&customer)))
}
