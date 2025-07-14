use axum::{
    Json,
    extract::{Path, Query, State},
};
use color_eyre::{Result, eyre::eyre};
use mongodb::bson::{Document, doc};
use serde::{Deserialize, Serialize};

use crate::{
    axum_error::{AxumError, AxumResult},
    middlewares::require_auth::UnauthorizedError,
    models::{Movie, movie::TMDBMovieData},
    routes::empty_string_as_none,
    routes::{Route, RouteProtectionLevel},
    state::AppState,
    tmdb_configuration::{TMDB_CONFIGURATION, movie_details, tv_series_details},
};

use utoipa_axum::routes;

const PATH: &str = "/api/movies/{movie_id}";

pub fn routes() -> Vec<Route> {
    vec![(routes!(get_movie), RouteProtectionLevel::Public)]
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMovieQuery {
    movie_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct GetMovieParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    refresh: Option<bool>,
}

/// Get a movie
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = Movie),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    params(
        ("refresh" = bool, Path, description = "Refresh movie details from TMDB", example = "false", ),
    ),
    tag = "Movies"
)]
pub async fn get_movie(
    State(state): State<AppState>,
    Path(movie_id): Path<String>,
    Query(params): Query<GetMovieParams>,
) -> AxumResult<Json<Movie>> {
    let movie = match params.refresh {
        Some(true) => add_movie_to_database(movie_id.clone(), state.clone()).await,
        _ => {
            let movie = state
                .db
                .collection::<Movie>("movies")
                .find_one(doc! {
                    "tmdb_id": &movie_id
                })
                .await?;
            if let Some(movie) = movie {
                Ok(movie)
            } else {
                add_movie_to_database(movie_id, state).await
            }
        }
    };

    match movie {
        Ok(movie) => Ok(Json(movie)),
        Err(e) => Err(AxumError::new(eyre!(
            "Failed to fetch movie details: {}",
            e
        ))),
    }
}

pub async fn add_movie_to_database(id: String, state: AppState) -> Result<Movie> {
    let movie: Movie = match id {
        id if id.starts_with('m') => {
            Movie::from_tmdb(
                TMDBMovieData::Movie(
                    movie_details(&TMDB_CONFIGURATION, id[1..].parse()?, Some("images"), None)
                        .await?,
                ),
                state.clone(),
            )
            .await?
        }
        id if id.starts_with('t') => {
            Movie::from_tmdb(
                TMDBMovieData::TV(
                    tv_series_details(&TMDB_CONFIGURATION, id[1..].parse()?, Some("images"), None)
                        .await?,
                ),
                state.clone(),
            )
            .await?
        }
        _ => return Err(eyre!("Invalid movie ID format: {}", id)),
    };

    let database_movie = state
        .db
        .collection::<Movie>("movies")
        .find_one_and_update(
            doc! {"tmdb_id": movie.tmdb_id.to_string()},
            doc! {"$set": Into::<Document>::into(movie.clone())},
        )
        .upsert(true)
        .return_document(mongodb::options::ReturnDocument::After)
        .await;

    match database_movie {
        Ok(Some(database_movie)) => Ok(database_movie),
        Err(e) => Err(eyre!("Failed to save movie to database, error: {e}")),
        _ => Err(eyre!("No movie returned")),
    }
}
