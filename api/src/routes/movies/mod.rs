use utoipa_axum::router::OpenApiRouter;

use crate::state::AppState;

pub mod movie_id;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().nest("/{movie_id}", movie_id::routes())
}
