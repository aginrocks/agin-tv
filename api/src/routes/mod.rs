mod auth;
mod health;
mod home;
pub mod movies;

use crate::{ApiDoc, middlewares::require_auth::require_auth, state::AppState};
use axum::middleware;
use utoipa::{OpenApi, ToSchema, schema};
use utoipa_axum::router::OpenApiRouter;

use serde::{Deserialize, Deserializer, Serialize, de};
use std::{fmt, str::FromStr};

pub fn api_routes() -> OpenApiRouter<AppState> {
    let public = OpenApiRouter::new()
        .nest("/health", health::routes())
        .nest("/auth", auth::routes());

    let auth = OpenApiRouter::new()
        .nest("/movies", movies::routes())
        .nest("/home", home::routes())
        .layer(middleware::from_fn(require_auth));

    auth.merge(public)
}

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::with_openapi(ApiDoc::openapi()).nest("/api", api_routes())
}

fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"error": "Unauthorized"}))]
pub struct UnauthorizedError {
    error: String,
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"error": "Not Found"}))]
pub struct NotFoundError {
    error: String,
}

#[derive(Serialize, ToSchema)]
pub struct GenericError {
    error: String,
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"success": true,"id": "60c72b2f9b1d8c001c8e4f5a"}))]
pub struct CreateSuccess {
    success: bool,
    id: String,
}
