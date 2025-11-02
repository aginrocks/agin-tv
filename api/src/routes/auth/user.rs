use axum::Json;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{axum_error::AxumResult, models::User, state::AppState};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(get_user))
}

/// Get get user info
#[utoipa::path(
    method(get),
    path = "/",
    responses(
        (status = OK, description = "Success", body = str)
    ),
    tag = "Auth"
)]
async fn get_user() -> AxumResult<Json<User>> {
    todo!()
}
