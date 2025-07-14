use axum::extract::State;
use utoipa_axum::routes;

use crate::{
    axum_error::AxumResult,
    routes::{Route, RouteProtectionLevel},
    state::AppState,
};

const PATH: &str = "/api/auth/start_session";

pub fn routes() -> Vec<Route> {
    vec![(routes!(start_session), RouteProtectionLevel::Public)]
}

/// Request link to identity provider
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = str)
    ),
    tag = "Auth"
)]
async fn start_session(State(state): State<AppState>) -> AxumResult<String> {
    Ok("placeholder".to_string())
}
