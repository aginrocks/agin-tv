mod health;
mod movies;

use crate::{ApiDoc, middlewares::require_auth, state::AppState};
use axum::middleware;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use serde::{Deserialize, Deserializer, de};
use std::{fmt, str::FromStr};

pub fn api_routes() -> OpenApiRouter<AppState> {
    let public = OpenApiRouter::new().nest("/health", health::routes());

    let auth = OpenApiRouter::new()
        .nest("/movies", movies::routes())
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
