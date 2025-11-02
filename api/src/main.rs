mod axum_error;
mod database;
mod init;
mod middlewares;
mod models;
mod mongo_id;
mod routes;
mod settings;
mod state;
mod tmdb_configuration;

use std::{net::SocketAddr, ops::Deref, sync::Arc};

use axum::{
    Router,
    error_handling::HandleErrorLayer,
    http::StatusCode,
    middleware::{self, from_fn},
    response::IntoResponse,
    routing::any,
};
use axum_oidc::{
    OidcAuthLayer, OidcClient, OidcLoginLayer, error::MiddlewareError, handle_oidc_redirect,
};
use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use openidconnect::{ClientId, IssuerUrl, core::CoreProviderMetadata};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_sessions::SessionManagerLayer;
use tower_sessions_redis_store::{RedisStore, fred::prelude::Pool};
use tracing::{Instrument, error, info, info_span, instrument, level_filters::LevelFilter};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    fmt::format::FmtSpan, layer::SubscriberExt as _, util::SubscriberInitExt as _,
};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as _};

use crate::{
    init::{
        create_oidc_client, init_axum, init_database, init_listener, init_redis, init_reqwest,
        init_session_store, init_tracing,
    },
    middlewares::{require_auth::require_auth, session_bearer_override},
    settings::Settings,
    state::AppState,
};

#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct GroupClaims {}
impl axum_oidc::AdditionalClaims for GroupClaims {}
impl openidconnect::AdditionalClaims for GroupClaims {}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    dotenvy::dotenv().ok();

    init_tracing().wrap_err("failed to set global tracing subscriber")?;

    info!(
        "Starting {} {}...",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    );

    let settings = Arc::new(Settings::try_load()?);

    let db = init_database(&settings).await?;

    let http_client = init_reqwest().wrap_err("failed to initialize HTTP client")?;

    let oidc_client = Arc::new(create_oidc_client(settings.clone()).await?);

    let redis_store = init_redis(&settings).await?;

    let app_state = AppState {
        db,
        http_client,
        settings: settings.clone(),
        oidc_client,
        redis_store: redis_store.clone(),
    };

    let session_layer = init_session_store(&settings, redis_store).await?;
    let app = init_axum(app_state, session_layer).await?;
    let listener = init_listener(&settings).await?;

    info!(
        "listening on {} ({})",
        listener
            .local_addr()
            .wrap_err("failed to get local address")?,
        settings.general.public_url
    );

    axum::serve(listener, app.into_make_service())
        .await
        .wrap_err("failed to run server")?;

    Ok(())
}
