use std::convert::Infallible;

use warp::Filter;

use crate::Environment;

pub fn with_env(
    env: Environment,
) -> impl Filter<Extract = (Environment,), Error = Infallible> + Clone {
    warp::any().map(move || env.clone())
}
