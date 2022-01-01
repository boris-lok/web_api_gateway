use warp::{Filter, Reply};
use warp::filters::BoxedFilter;

use crate::core::middleware::with_env;
use crate::Environment;
use crate::user::handler::v1::create_user_handler;

pub fn routes(env: Environment) -> BoxedFilter<(impl Reply,)> {
    let login_route = warp::path!("api" / "v1" / "users")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_env(env))
        .and_then(create_user_handler);

    let routes = login_route;
    routes.boxed()
}
