use std::str::FromStr;
use std::sync::Arc;

use axum::extract::RawQuery;
use axum::{response::IntoResponse, routing::get, Extension, Router};
use reqwest::cookie::{Cookie, CookieStore};
use tauri::{Manager, Url};
use tokio::sync::{oneshot, Mutex};

use crate::helpers::build_url;
use crate::state::AppState;

async fn authorize(
    RawQuery(query): RawQuery,
    Extension(handle): Extension<tauri::AppHandle>,
    Extension(shutdown_tx): Extension<Arc<Mutex<Option<oneshot::Sender<()>>>>>,
) -> impl IntoResponse {
    let state = handle.state::<AppState>();

    let mut url = match build_url("/auth/login") {
        Ok(url) => url,
        Err(e) => {
            eprintln!("Failed to build URL: {e}");
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to build URL",
            )
                .into_response();
        }
    };

    if let Some(query) = query.as_deref() {
        url.set_query(Some(query));
    }

    let res = state
        .http_client
        .post(url.as_str())
        .send()
        .await
        .expect("Failed to send request");

    dbg!(&res);

    if !res.status().is_success() {
        return (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Authentication failed",
        )
            .into_response();
    }

    // state.http_client.cookie_store().unwrap().clear();

    if let Some(tx) = shutdown_tx.lock().await.take() {
        let _ = tx.send(());
    }

    let contents = include_str!("../resources/oauth.html");
    axum::response::Html(contents).into_response()
}

pub async fn run_server(handle: tauri::AppHandle) -> Result<&'static str, std::io::Error> {
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
        .await?;

    Ok("XD")
}
