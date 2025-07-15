use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{Json, extract::State};
use openidconnect::{CsrfToken, Nonce, RedirectUrl, Scope, core::CoreAuthenticationFlow};
use serde::Serialize;
use tower_sessions::Session;
use utoipa::ToSchema;
use utoipa_axum::routes;

use crate::{
    axum_error::AxumResult,
    routes::{Route, RouteProtectionLevel},
    state::AppState,
};

const PATH: &str = "/api/auth/start_session";

pub fn routes() -> Vec<Route> {
    vec![(routes!(start_session), RouteProtectionLevel::Public)]
}

#[derive(ToSchema, Serialize)]
struct StartSessionResponse {
    pub auth_url: String,
}

/// Request link to identity provider
#[utoipa::path(
    method(post),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = StartSessionResponse)
    ),
    tag = "Auth"
)]
async fn start_session(
    State(state): State<AppState>,
    session: Session,
) -> AxumResult<Json<StartSessionResponse>> {
    let csrf_token = CsrfToken::new_random();

    session.insert("csrf_token", csrf_token.clone()).await?;

    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4321);
    let redirect_url: String = format!("http://{socket_addr}/callback").to_string();

    let (auth_url, _, _) = state
        .oidc_client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            || csrf_token,
            Nonce::new_random,
        )
        .set_redirect_uri(std::borrow::Cow::Owned(
            RedirectUrl::new(redirect_url).unwrap(),
        ))
        // Set the desired scopes.
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("openid".to_string()))
        .url();

    Ok(Json(StartSessionResponse {
        auth_url: auth_url.to_string(),
    }))
}
