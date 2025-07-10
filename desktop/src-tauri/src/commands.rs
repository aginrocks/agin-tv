use openidconnect::{core::CoreAuthenticationFlow, Nonce, Scope};
use tauri::Manager;

use crate::{oidc::run_server, state::AppState};

#[tauri::command]
pub async fn authenticate(handle: tauri::AppHandle) {
    let auth = handle.state::<AppState>();

    let csrf_token = auth.csrf_token.clone();

    let (auth_url, _, _) = auth
        .client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            || csrf_token,
            Nonce::new_random,
        )
        // Set the desired scopes.
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(auth.pkce.0.clone())
        .url();

    let _server_handle =
        tauri::async_runtime::spawn(async move { run_server(handle.clone()).await });

    open::that(auth_url.to_string()).unwrap();
}
