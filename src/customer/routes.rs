use warp::{filters::BoxedFilter, Filter, Reply};
use crate::customer::handlers::v1::{get, create};
use crate::utils::env::Env;

use crate::utils::middlewares::with_env::with_env;

pub fn routes(env: Env) -> BoxedFilter<(impl Reply, )> {
    let get_route = warp::path!("api" / "v1" / "customers" / u64)
        .and(warp::get())
        .and(with_env(env.clone()))
        .and_then(get);

    let create_route = warp::path!("api" / "v1" / "customers")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_env(env))
        .and_then(create);

    let routes = get_route.or(create_route);

    routes.boxed()
}
