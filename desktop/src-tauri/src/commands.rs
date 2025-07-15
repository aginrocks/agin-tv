use serde::Deserialize;
use tauri::Manager;

use crate::{helpers::build_url, state::AppState};

#[derive(Deserialize)]
struct StartSessionResponse {
    pub auth_url: String,
}

#[tauri::command]
pub async fn authenticate(handle: tauri::AppHandle) {
    let state = handle.state::<AppState>();

    let http_client = state.http_client.clone();

    let url = match build_url("/auth/start_session") {
        Ok(url) => url,
        Err(e) => {
            eprintln!("Failed to build URL: {e}");
            return;
        }
    };

    dbg!(&url.to_string());

    let res = http_client
        .post(url)
        .send()
        .await
        .expect("Failed to start session")
        .json::<StartSessionResponse>()
        .await
        .expect("Failed to parse response");

    open::that(res.auth_url).unwrap();
}
