use axum::{
    Json,
    extract::{Path, Query, RawPathParams, State},
};
use color_eyre::{Result, eyre::eyre};
use mongodb::bson::{Document, doc};
use serde::{Deserialize, Serialize};
use tmdb::apis::default_api::movie_details;

use crate::{
    axum_error::{AxumError, AxumResult},
    middlewares::require_auth::UnauthorizedError,
    models::{Movie, TMDBMovieData},
    routes::{Route, RouteProtectionLevel},
    state::AppState,
    tmdb_configuration::TMDB_CONFIGURATION,
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
    State(state): State<AppState>,
    Path(movie_id): Path<i32>,
) -> AxumResult<Json<Movie>> {
    let movie = state
        .db
        .collection::<Movie>("movies")
        .find_one(doc! {
            "tmdb_id": movie_id
        })
        .await?;

    if let Some(movie) = movie {
        Ok(Json(movie))
    } else {
        let movie = app_movie_to_database(movie_id, state).await;

        match movie {
            Ok(movie) => Ok(Json(movie)),
            Err(e) => Err(AxumError::new(eyre!(
                "Failed to fetch movie details: {}",
                e
            ))),
        }
    }
}

pub async fn app_movie_to_database(id: i32, state: AppState) -> Result<Movie> {
    let movie = movie_details(&TMDB_CONFIGURATION, id, Some("images"), None).await;
    // dbg!(&movie);

    if let Err(e) = movie {
        return Err(eyre!("Failed to fetch movie details: {}", e));
    }

    let movie = movie.unwrap();
    let movie = Movie::from_tmdb(TMDBMovieData::Movie(movie));

    dbg!(&movie.tmdb_id.to_string());

    let database_movie = state
        .db
        .collection::<Movie>("movies")
        .find_one_and_update(
            doc! {"tmdb_id": movie.tmdb_id.to_string()},
            // doc! {"$set": Into::<Document>::into(movie.clone())},
            doc! {"$set": {"name": "test"}},
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
