use std::convert::Infallible;

use warp::Filter;

use crate::utils::env::Env;

pub fn with_env(env: Env) -> impl Filter<Extract = (Env,), Error = Infallible> + Clone {
    warp::any().map(move || env.clone())
}
