use std::env;

use serde::Deserialize;
use tauri::{Manager, Url};
use tauri_plugin_store::StoreExt;

use crate::{helpers::build_url, oidc::run_server, state::AppState};

#[derive(Deserialize)]
struct StartSessionJson {
    pub auth_url: String,
}

#[derive(Deserialize)]
struct SessionJson {
    pub token: String,
}

#[derive(Deserialize)]
enum StartSessionResponse {
    New(StartSessionJson),
    Old(SessionJson),
}

#[tauri::command]
pub async fn authenticate(handle: tauri::AppHandle) -> Result<Option<String>, String> {
    dbg!("starting authentication process");
    let state = handle.state::<AppState>();

    if let Some(abort_handle) = state.abort_handle.write().await.take() {
        dbg!("aborting previous session");
        abort_handle.abort();
    }

    let store = handle.store("store.json").unwrap();

    let url = build_url("/auth/start_session")?;

    let res = state
        .http_client
        .post(url)
        .send()
        .await
        .map_err(|_| "failed to start session")?;

    let cookie = res
        .cookies()
        .find(|cookie| cookie.name() == "id")
        .map(|c| c.value().to_string());

    let json = res
        .json::<StartSessionResponse>()
        .await
        .expect("Failed to parse response");

    if let StartSessionResponse::New(json) = json {
        store.set("token", cookie.clone());

        let server_handle = tokio::spawn(run_server(handle.clone()));

        let abort_handle = server_handle.abort_handle();

        {
            let mut state_handle = state.abort_handle.write().await;
            *state_handle = Some(abort_handle);
        }

        //TODO: check if browser is opened
        dbg!("opening browser");
        open::that(json.auth_url).unwrap();

        match server_handle.await {
            Ok(_) => Ok(cookie),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Unexpected response".to_string())
    }
}

#[tauri::command]
pub async fn cancel_authentication(handle: tauri::AppHandle) -> Result<(), String> {
    let state = handle.state::<AppState>();

    if let Some(abort_handle) = state.abort_handle.write().await.take() {
        abort_handle.abort();
    }

    Ok(())
}
