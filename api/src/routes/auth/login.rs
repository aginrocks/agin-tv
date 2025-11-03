use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::extract::{Query, State};
use bson::doc;
use color_eyre::eyre::eyre;
use openidconnect::{AuthorizationCode, CsrfToken, OAuth2TokenResponse, RedirectUrl};
use serde::Deserialize;
use tower_sessions::Session;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::{AxumError, AxumResult},
    middlewares::GroupClaims,
    models::{PartialToken, User},
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(login))
}

#[derive(Deserialize)]
struct CallbackQuery {
    code: AuthorizationCode,
    state: CsrfToken,
}

/// Login endpoint that handles the OIDC login flow
#[utoipa::path(
    method(post),
    path = "/",
    responses(
        (status = OK, description = "Success", body = String)
    ),
    tag = "Auth"
)]
async fn login(
    State(state): State<AppState>,
    session: Session,
    query: Query<CallbackQuery>,
) -> AxumResult<String> {
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

    let user_info: openidconnect::UserInfoClaims<
        GroupClaims,
        openidconnect::core::CoreGenderClaim,
    > = client_with_redirect
        .user_info(token.access_token().clone(), None)
        .map_err(|e| AxumError::internal_server_error(eyre!("Failed to get user info: {}", e)))?
        .request_async(&state.http_client)
        .await?;

    let user = state
        .db
        .collection::<User>("users")
        .find_one_and_update(
            doc! { "sub": user_info.subject().as_str() },
            doc! {
                "$set": {
                    "sub": user_info.subject().as_str(),
                    "name": user_info.name()
                        .and_then(|n| n.get(None))
                        .map(|n| n.as_str())
                        .unwrap_or("Unnamed User"),
                    "email": user_info.email()
                    .map(|n| n.as_str())
                    .unwrap_or(""),
                }
            },
        )
        .upsert(true)
        .return_document(mongodb::options::ReturnDocument::After)
        .await?;

    state
        .db
        .collection("tokens")
        .insert_one(PartialToken {
            token: token.access_token().secret().to_string(),
            user_id: user.unwrap().id,
        })
        .await?;

    session
        .insert("access_token", token.access_token().secret())
        .await?;

    Ok(token.access_token().secret().to_owned())
}
