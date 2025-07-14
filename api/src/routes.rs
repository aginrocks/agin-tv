use serde::{Deserialize, Deserializer, de};
use std::{fmt, str::FromStr};
use utoipa_axum::router::UtoipaMethodRouter;

use crate::state::AppState;

mod health;
pub mod movies;

pub fn routes() -> Vec<Route> {
    [movies::routes(), health::routes()].concat()
}

#[derive(Clone)]
pub enum RouteProtectionLevel {
    Public,
    Redirect,
    Authenticated,
}

type Route = (UtoipaMethodRouter<AppState>, RouteProtectionLevel);

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
