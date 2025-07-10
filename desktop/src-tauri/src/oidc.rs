use std::{env, sync::Arc};

use axum::{extract::Query, response::IntoResponse, routing::get, Extension, Router};
use openidconnect::{
    AuthUrl, Client, ClientId, ClientSecret, IssuerUrl, JsonWebKeySet, RedirectUrl, TokenUrl,
};
use openidconnect::{AuthorizationCode, CsrfToken, PkceCodeVerifier};
use serde::Deserialize;
use tauri::Manager;
use tokio::sync::{oneshot, Mutex};

use crate::state::{AppState, OidcClient};

pub fn create_client(redirect_url: RedirectUrl) -> OidcClient {
    let client_id = ClientId::new(env::var("CLIENT_ID").expect("No cliend id"));

    let auth_url =
        AuthUrl::new(env::var("AUTH_URL").expect("No auth url")).expect("Invalid AUTH_URL!");

    let token_url =
        TokenUrl::new(env::var("TOKEN_URL").expect("No token url")).expect("Invalid TOKEN_URL!");

    let client_secret = ClientSecret::new(env::var("CLIENT_SECRET").expect("No client secret"));

    Client::new(
        client_id,
        IssuerUrl::new(env::var("ISSUER_URL").expect("No token url")).expect("Invalid ISSUER_URL!"),
        JsonWebKeySet::new(vec![]),
    )
    .set_redirect_uri(redirect_url)
    .set_auth_uri(auth_url)
    .set_token_uri(token_url)
    .set_client_secret(client_secret)
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

    let _token = auth
        .client
        .exchange_code(query.code.clone())
        .set_pkce_verifier(PkceCodeVerifier::new(auth.pkce.1.clone()))
        .request_async(&auth.http_client)
        .await
        .unwrap();

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
