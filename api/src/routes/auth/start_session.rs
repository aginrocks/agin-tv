use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{
    Json,
    extract::{Request, State},
};
use openidconnect::{CsrfToken, Nonce, RedirectUrl, Scope, core::CoreAuthenticationFlow};
use serde::Serialize;
use tower_sessions::Session;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{axum_error::AxumResult, state::AppState};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(start_session))
}

#[derive(ToSchema, Serialize)]
struct StartSessionJson {
    pub auth_url: String,
}

#[derive(ToSchema, Serialize)]
struct SessionJson {
    pub token: String,
}

#[derive(ToSchema, Serialize)]
enum StartSessionResponse {
    New(StartSessionJson),
    Old(SessionJson),
}

/// Request link to identity provider
#[utoipa::path(
    method(post),
    path = "/",
    responses(
        (status = OK, description = "Success", body = StartSessionResponse)
    ),
    tag = "Auth"
)]
async fn start_session(
    State(state): State<AppState>,
    session: Session,
    request: Request,
) -> AxumResult<Json<StartSessionResponse>> {
    dbg!(request.headers());

    if let Some(token) = session.get::<String>("access_token").await? {
        return Ok(Json(StartSessionResponse::Old(SessionJson { token })));
    }

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
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("openid".to_string()))
        .url();

    Ok(Json(StartSessionResponse::New(StartSessionJson {
        auth_url: auth_url.to_string(),
    })))
}
