use axum::{Json, extract::State};
use bson::doc;
use futures::stream::TryStreamExt;
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use visible::StructFields;

use crate::{axum_error::AxumResult, models::Movie, routes::UnauthorizedError, state::AppState};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(home))
}

#[derive(Serialize, ToSchema)]
#[StructFields(pub)]
pub struct CarouselMovie {
    tmdb_id: String,
    name: String,
    description: String,
    background_url: Option<String>,
    logo_url: Option<String>,
}

#[derive(Serialize, ToSchema)]
struct HomeResponse {
    carousel: Vec<CarouselMovie>,
}

/// Get homescreen data
#[utoipa::path(
    method(get),
    path = "/",
    responses(
        (status = OK, description = "Success", body = HomeResponse),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Home"
)]
async fn home(State(state): State<AppState>) -> AxumResult<Json<HomeResponse>> {
    let movies = state
        .db
        .collection::<Movie>("movies")
        .find(doc! {})
        .limit(5)
        .await?;

    let movies = movies.try_collect::<Vec<_>>().await?;

    let carousel: Vec<CarouselMovie> = movies
        .iter()
        .map(|movie| CarouselMovie {
            tmdb_id: movie.tmdb_id.to_string(),
            name: movie.name.clone(),
            description: movie.description.clone(),
            background_url: movie.background_url.clone(),
            logo_url: movie.logo_url.clone(),
        })
        .collect();

    Ok(Json(HomeResponse { carousel }))
}
