use serde::Deserialize;
use tauri::Manager;

use crate::{helpers::build_url, oidc::run_server, state::AppState};

#[derive(Deserialize)]
struct StartSessionResponse {
    pub auth_url: String,
}

#[tauri::command]
pub async fn authenticate(handle: tauri::AppHandle) -> Result<Option<String>, String> {
    let state = handle.state::<AppState>();

    let url = build_url("/auth/start_session")?;

    let res = state
        .http_client
        .post(url)
        .send()
        .await
        .expect("Failed to start session");

    let cookie = res
        .cookies()
        .find(|cookie| cookie.name() == "id")
        .map(|c| c.value().to_string());

    let json = res
        .json::<StartSessionResponse>()
        .await
        .expect("Failed to parse response");

    let server_handle =
        tauri::async_runtime::spawn(async move { run_server(handle.clone()).await });

    open::that(json.auth_url).unwrap();

    match server_handle.await.expect("Failed to run server") {
        Ok(_) => Ok(cookie),
        Err(e) => Err(e.to_string()),
    }
}
