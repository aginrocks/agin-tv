use utoipa_axum::routes;

use crate::{axum_error::AxumResult, _routes::RouteProtectionLevel};

use super::Route;

const PATH: &str = "/auth/user";

pub fn routes() -> Vec<Route> {
    vec![(routes!(get_user), RouteProtectionLevel::Authenticated)]
}

/// Get get user info
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = str)
    ),
    tag = "Auth"
)]
async fn get_user() -> AxumResult<Json> {}
