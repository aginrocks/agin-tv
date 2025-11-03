use crate::{
    axum_error::AxumResult,
    models::{Movie, movie::TMDBMovieData},
    routes::{UnauthorizedError, movies::movie_id::MovieResponse},
    state::AppState,
    tmdb_configuration::{TMDB_CONFIGURATION, movie_details, tv_series_details},
};
use axum::{
    Json,
    extract::{Query, State},
};
use serde::Deserialize;
use tmdb::apis::default_api::search_multi;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(search))
}

#[derive(Deserialize)]
struct SearchParams {
    query: String,
    #[serde(default)]
    language: Option<String>,
}

/// Search endpoint
#[utoipa::path(
    method(get),
    path = "/",
    responses(
        (status = OK, description = "Success", body = Vec<Movie>),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Bad Request", body = String, content_type = "application/json")
    ),
    tag = "Search"
)]
async fn search(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> AxumResult<Json<Vec<MovieResponse>>> {
    let search_results = search_multi(
        &TMDB_CONFIGURATION,
        params.query.as_str(),
        Some(true),
        params.language.as_deref(),
        None,
    )
    .await?;

    let filtered_results = search_results
        .results
        .unwrap_or_default()
        .into_iter()
        .filter(|item| {
            item.media_type
                .as_ref()
                .map(|mt| mt == "tv" || mt == "movie")
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();

    let mut movies = Vec::new();

    for item in filtered_results {
        let media_type = item.media_type.as_deref().unwrap_or("");
        let id = item.id.unwrap_or(0);

        let movie_result = match media_type {
            "movie" => {
                let details = movie_details(&TMDB_CONFIGURATION, id, Some("images"), None).await?;
                Movie::from_tmdb(TMDBMovieData::Movie(details), state.clone()).await
            }
            "tv" => {
                let details =
                    tv_series_details(&TMDB_CONFIGURATION, id, Some("images"), None).await?;
                Movie::from_tmdb(TMDBMovieData::TV(details), state.clone()).await
            }
            _ => continue,
        };

        match movie_result {
            Ok(movie) => {
                if let Ok(response) = movie.populate_genres(state.clone()).await {
                    movies.push(response);
                }
            }
            Err(e) => {
                tracing::warn!("Failed to convert item {}: {}", id, e);
                continue;
            }
        }
    }

    Ok(Json(movies))
}
