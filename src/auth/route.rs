use warp::{Filter, Reply};
use warp::filters::BoxedFilter;

use crate::auth::handler::v1::login_handler;
use crate::core::middleware::with_env;
use crate::Environment;

pub fn routes(env: Environment) -> BoxedFilter<(impl Reply,)> {
    let login_route = warp::path!("api" / "v1" / "login")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_env(env))
        .and_then(login_handler);

    let routes = login_route;
    routes.boxed()
}
