use utoipa_axum::router::UtoipaMethodRouter;

use crate::state::AppState;

pub fn routes() -> Vec<Route> {
    // [api::routes()].concat()
    vec![]
}

#[derive(Clone)]
pub enum RouteProtectionLevel {
    Public,
    Redirect,
    Authenticated,
}

type Route = (UtoipaMethodRouter<AppState>, RouteProtectionLevel);
