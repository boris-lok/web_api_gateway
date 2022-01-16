use warp::{Filter, Reply};
use warp::filters::BoxedFilter;
use warp_reverse_proxy::reverse_proxy_filter;

use crate::{Environment, WebResult};
use crate::auth::json::claims::Claims;
use crate::core::middlewares::authorization::authenticated_from_cookie;

pub fn routes(env: Environment) -> BoxedFilter<(impl Reply,)> {
    let customer_routes = warp::path!("api" / "v1" / "customers" / ..)
        .and(authenticated_from_cookie(env))
        .and(reverse_proxy_filter(
            "".to_string(),
            "http://127.0.0.1:3031".to_string(),
        ))
        .and_then(log_response);

    customer_routes.boxed()
}

async fn log_response(
    claims: Claims,
    response: warp::http::Response<warp::hyper::body::Bytes>,
) -> WebResult<impl Reply> {
    Ok(response)
}
