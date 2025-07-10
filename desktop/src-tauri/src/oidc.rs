use std::{env, sync::Arc};

use axum::{extract::Query, response::IntoResponse, routing::get, Extension, Router};
use openidconnect::core::CoreProviderMetadata;
use openidconnect::{AuthorizationCode, CsrfToken, PkceCodeVerifier};
use openidconnect::{Client, ClientId, ClientSecret, IssuerUrl, RedirectUrl};
use serde::Deserialize;
use tauri::Manager;
use tokio::sync::{oneshot, Mutex};

use crate::state::{AppState, OidcClient};

pub fn create_client(redirect_url: RedirectUrl) -> OidcClient {
    let client_id = ClientId::new(env::var("CLIENT_ID").expect("No cliend id"));

    let client_secret = ClientSecret::new(env::var("CLIENT_SECRET").expect("No client secret"));

    let http_client = openidconnect::reqwest::blocking::ClientBuilder::new()
        // Following redirects opens the client up to SSRF vulnerabilities.
        .redirect(openidconnect::reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

    let provider_metadata = CoreProviderMetadata::discover(
        &IssuerUrl::new(env::var("ISSUER_URL").expect("No issuer url"))
            .expect("Invalid ISSUER_URL!"),
        &http_client,
    )
    .expect("Failed to discover provider metadata");

    Client::from_provider_metadata(provider_metadata, client_id, Some(client_secret))
        .set_redirect_uri(redirect_url)
}

#[derive(Deserialize)]
struct CallbackQuery {
    code: AuthorizationCode,
    state: CsrfToken,
}

async fn authorize(
    query: Query<CallbackQuery>,
    Extension(handle): Extension<tauri::AppHandle>,
    Extension(shutdown_tx): Extension<Arc<Mutex<Option<oneshot::Sender<()>>>>>,
) -> impl IntoResponse {
    let auth = handle.state::<AppState>();

    if query.state.secret() != auth.csrf_token.secret() {
        println!("Suspected Man in the Middle attack!");
        return "authorized".to_string().into_response(); // never let them know your next move
    }

    let exchange = auth.client.exchange_code(query.code.clone());

    if let Err(e) = exchange {
        eprintln!("Failed to exchange code: {e}");
        return (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to exchange code",
        )
            .into_response();
    }
    let _token = exchange
        .unwrap()
        .set_pkce_verifier(PkceCodeVerifier::new(auth.pkce.1.clone()))
        .request_async(&auth.http_client)
        .await;

    // Signal the server to shutdown
    if let Some(tx) = shutdown_tx.lock().await.take() {
        let _ = tx.send(());
    }

    // Serve the oauth.html file as the response
    let html_path = std::path::Path::new("resources/oauth.html");
    match tokio::fs::read_to_string(html_path).await {
        Ok(contents) => axum::response::Html(contents).into_response(),
        Err(_) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to load oauth.html",
        )
            .into_response(),
    }
}

pub async fn run_server(handle: tauri::AppHandle) -> Result<(), std::io::Error> {
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    let shutdown_tx = Arc::new(Mutex::new(Some(shutdown_tx)));

    let app = Router::new()
        .route("/callback", get(authorize))
        .layer(Extension(handle.clone()))
        .layer(Extension(shutdown_tx));

    let socket_addr = handle.state::<AppState>().socket_addr;
    let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();

    // Run the server with graceful shutdown
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(async {
            shutdown_rx.await.ok();
        })
        .await
}
