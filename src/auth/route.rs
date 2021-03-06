use warp::{Filter, Reply};
use warp::filters::BoxedFilter;

use crate::auth::handlers::v1::{login_handler, logout_handler, renew_handler};
use crate::core::middlewares::authorization::{authenticated_from_cookie};
use crate::core::middlewares::with_env::with_env;
use crate::Environment;

pub fn routes(env: Environment) -> BoxedFilter<(impl Reply,)> {
    let login_route = warp::path!("api" / "v1" / "login")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_env(env.clone()))
        .and_then(login_handler);

    let logout_route = warp::path!("api" / "v1" / "logout")
        .and(warp::post())
        .and(authenticated_from_cookie(env.clone()))
        .and(with_env(env.clone()))
        .and_then(logout_handler);

    let renew_route = warp::path!("api" / "v1" / "token" / "renew")
        .and(warp::post())
        .and(authenticated_from_cookie(env.clone()))
        .and(with_env(env))
        .and_then(renew_handler);

    let routes = login_route.or(logout_route).or(renew_route);
    routes.boxed()
}
