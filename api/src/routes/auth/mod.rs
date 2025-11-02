use axum::middleware;
use utoipa_axum::router::OpenApiRouter;

use crate::{middlewares::require_auth::require_auth, state::AppState};

mod login;
mod start_session;
mod user;

pub fn routes() -> OpenApiRouter<AppState> {
    let public = OpenApiRouter::new()
        .nest("/login", login::routes())
        .nest("/start_session", start_session::routes());

    let auth = OpenApiRouter::new()
        .nest("/user", user::routes())
        .layer(middleware::from_fn(require_auth));

    public.merge(auth)
}
