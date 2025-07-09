use utoipa_axum::router::UtoipaMethodRouter;

use crate::state::AppState;

mod health;
mod movies;

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
