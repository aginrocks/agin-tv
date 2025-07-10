use axum::{Extension, Json, extract::State};

use crate::{
    axum_error::AxumResult,
    middlewares::require_auth::{UnauthorizedError, UserData},
    routes::{Route, RouteProtectionLevel},
    state::AppState,
};

use utoipa_axum::routes;

const PATH: &str = "/api/movies/{movie_id}";

pub fn routes() -> Vec<Route> {
    vec![(routes!(get_movie), RouteProtectionLevel::Authenticated)]
}

/// Get a movie
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        // (status = OK, description = "Success", body = Vec<Organization>),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Movies"
)]
pub async fn get_movie(
    Extension(user): Extension<UserData>,
    State(state): State<AppState>,
) -> AxumResult<()> {
    // let movie_id = mongo_id::parse_object_id(&movie_id)?;
    // let movie = state
    //     .movies
    //     .find_one(doc! { "_id": movie_id }, None)
    //     .await?
    //     .ok_or(ApiError::NotFound)?;

    Ok(())
}
