use crate::{
    middlewares::GroupClaims,
    routes::routes,
    settings::Settings,
    state::{AppState, OidcClient},
};
use axum::{
    Extension, Json, Router, error_handling::HandleErrorLayer, response::IntoResponse, routing::get,
};
use axum_oidc::{OidcAuthLayer, error::MiddlewareError};
use color_eyre::eyre::Result;
use fred::prelude::{ClientLike, Config, Pool};
use http::StatusCode;
use mongodb::{Client, Database};
use openidconnect::{ClientId, IssuerUrl, core::CoreProviderMetadata};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_sessions::{
    Expiry, SessionManagerLayer,
    cookie::{SameSite, time::Duration},
};
use tower_sessions_redis_store::RedisStore;
use tracing::{Instrument, error, info, info_span, instrument, level_filters::LevelFilter};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    fmt::format::FmtSpan, layer::SubscriberExt as _, util::SubscriberInitExt as _,
};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as _};

pub async fn init_database(settings: &Settings) -> Result<Database> {
    let client = Client::with_uri_str(&settings.db.connection_string).await?;
    let database = client.database(&settings.db.database_name);

    Ok(database)
}

pub async fn init_redis(settings: &Settings) -> Result<Pool> {
    let config = Config::from_url(&settings.redis.connection_string)?;
    let poll = Pool::new(config, None, None, None, 6)?;

    poll.connect();
    poll.wait_for_connect().await?;

    Ok(poll)
}

pub async fn init_session_store(
    settings: &Settings,
    redis_store: Pool,
) -> Result<SessionManagerLayer<RedisStore<Pool>>> {
    let session_store = RedisStore::<Pool>::new(redis_store);

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::days(7)));

    Ok(session_layer)
}

pub fn init_tracing() -> Result<()> {
    tracing_subscriber::Registry::default()
        .with(tracing_subscriber::fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
        .with(ErrorLayer::default())
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .with_env_var("RUST_LOG")
                .from_env()?,
        )
        .try_init()?;

    Ok(())
}

pub fn init_reqwest() -> Result<reqwest::Client, reqwest::Error> {
    reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
}

pub async fn create_oidc_client(settings: Arc<Settings>) -> Result<OidcClient> {
    let client_id = ClientId::new(settings.oidc.client_id.to_string());

    let client_secret = settings.oidc.client_secret.clone();

    let http_client = openidconnect::reqwest::ClientBuilder::new()
        .redirect(openidconnect::reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

    let provider_metadata = CoreProviderMetadata::discover_async(
        IssuerUrl::new(settings.oidc.issuer.to_string())?,
        &http_client,
    )
    .await?;

    Ok(openidconnect::Client::from_provider_metadata(
        provider_metadata,
        client_id,
        client_secret,
    ))
}

#[instrument(skip(state, session_layer))]
pub async fn init_axum(
    state: AppState,
    session_layer: SessionManagerLayer<RedisStore<Pool>>,
) -> Result<Router> {
    let app_url = format!(
        "{}/api/login/callback",
        state
            .settings
            .general
            .public_url
            .to_string()
            .trim_end_matches('/')
    );

    let mut oidc_client = axum_oidc::OidcClient::<GroupClaims>::builder()
        .with_default_http_client()
        .with_redirect_url(app_url.parse()?)
        .with_client_id(state.settings.oidc.client_id.as_str())
        .add_scope("profile")
        .add_scope("email");

    if let Some(client_secret) = state.settings.oidc.client_secret.as_ref() {
        oidc_client = oidc_client.with_client_secret(client_secret.secret().clone());
    }

    let oidc_client = oidc_client
        .discover(state.settings.oidc.issuer.to_string())
        .instrument(info_span!("oidc_discover"))
        .await?
        .build();

    let oidc_auth_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e: MiddlewareError| async {
            error!(error = ?e, "An error occurred in OIDC auth middleware");
            e.into_response()
        }))
        .layer(OidcAuthLayer::new(oidc_client));

    let router = routes();

    let (router, api) = router.with_state(state.clone()).split_for_parts();

    let openapi_prefix = "/apidoc";
    let spec_name = "/openapi.json";

    let docs = Router::new()
        .merge(Redoc::with_url("/redoc", api.clone()))
        .merge(RapiDoc::new(format!("{openapi_prefix}{spec_name}")).path("/rapidoc"))
        .merge(Scalar::with_url("/scalar", api.clone()))
        .route(spec_name, get(|| async move { Json(api) }));

    let router = router
        .nest(openapi_prefix, docs)
        .layer(Extension(state))
        .layer(oidc_auth_service)
        .layer(session_layer)
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin(vec![
                    "http://localhost:5173".parse().unwrap(),
                    "tauri://localhost".parse().unwrap(),
                ])
                .allow_methods([
                    axum::http::Method::GET,
                    axum::http::Method::POST,
                    axum::http::Method::OPTIONS,
                    axum::http::Method::PUT,
                    axum::http::Method::DELETE,
                ])
                .allow_headers([
                    axum::http::header::AUTHORIZATION,
                    axum::http::header::CONTENT_TYPE,
                    axum::http::header::COOKIE,
                ])
                .allow_credentials(true),
        )
        .fallback(|| async { (StatusCode::NOT_FOUND, "Not found").into_response() });

    Ok(router)
}

pub async fn init_listener(settings: &Settings) -> Result<TcpListener> {
    let addr: Vec<SocketAddr> = settings.general.listen_address.clone().into();

    Ok(TcpListener::bind(addr.as_slice()).await?)
}
