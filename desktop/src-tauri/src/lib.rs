mod commands;
mod helpers;
mod oidc;
mod state;

use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use color_eyre::Result;

use crate::state::AppState;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! You've been greeted from Rust!")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<()> {
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4321); // or any other port

    let cookie_store = Arc::new(reqwest::cookie::Jar::default());

    let http_client = reqwest::Client::builder()
        .user_agent("agin-tv/desktop")
        .cookie_provider(cookie_store.clone())
        .build()?;

    let state = AppState {
        http_client,
        cookie_store,
        socket_addr,
        abort_handle: Arc::new(tokio::sync::RwLock::new(None)),
    };

    tauri::Builder::default()
        .manage(state)
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![
            commands::authenticate,
            commands::cancel_authentication,
            greet,
        ])
        .run(tauri::generate_context!())?;
    Ok(())
}
