use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::extract::{Query, State};
use color_eyre::eyre::eyre;
use openidconnect::{AuthorizationCode, CsrfToken, OAuth2TokenResponse, RedirectUrl};
use serde::Deserialize;
use tower_sessions::Session;
use utoipa_axum::routes;

use crate::{
    axum_error::{AxumError, AxumResult},
    routes::RouteProtectionLevel,
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/auth/login";

pub fn routes() -> Vec<Route> {
    vec![(routes!(login), RouteProtectionLevel::Public)]
}

#[derive(Deserialize)]
struct CallbackQuery {
    code: AuthorizationCode,
    state: CsrfToken,
}

/// Login endpoint that handles the OIDC login flow
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = str)
    ),
    tag = "Auth"
)]
async fn login(
    State(state): State<AppState>,
    session: Session,
    query: Query<CallbackQuery>,
) -> AxumResult<&'static str> {
    let csrf_token = session.get::<CsrfToken>("csrf_token").await?;

    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4321);
    let redirect_url: String = format!("http://{socket_addr}/callback").to_string();

    if csrf_token.is_none() {
        return Err(AxumError::unauthorized(eyre!(
            "No CSRF token found in session"
        )));
    }

    if query.state.secret() != csrf_token.unwrap().secret() {
        return Err(AxumError::bad_request(eyre!("Invalid CSRF token")));
    }

    let redirect_url = RedirectUrl::new(redirect_url).unwrap();

    let client_with_redirect = state
        .oidc_client
        .as_ref()
        .clone()
        .set_redirect_uri(redirect_url);

    let exchange = client_with_redirect.exchange_code(query.code.clone());

    let token = match exchange {
        Ok(exchange) => exchange.request_async(&state.http_client).await?,
        Err(e) => return Err(AxumError::internal_server_error(eyre!(e))),
    };

    session
        .insert("access_token", token.access_token().secret())
        .await?;

    Ok("ok")
}
