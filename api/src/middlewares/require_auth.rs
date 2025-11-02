use std::ops::Deref;

use axum::{
    Extension,
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use axum_oidc::OidcClaims;
use color_eyre::eyre::{self, ContextCompat};
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::ReturnDocument,
};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use utoipa::ToSchema;

use crate::{
    GroupClaims,
    axum_error::{AxumError, AxumResult},
    models::User,
    state::AppState,
};

/// User data type for request extensions
#[derive(Clone, Debug, Serialize, ToSchema, Deserialize)]
pub struct UserData(pub User);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserId(pub ObjectId);

impl Deref for UserId {
    type Target = ObjectId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// TODO: FIx that shit
/// Middleware that ensures the user is authenticated
pub async fn require_auth(
    claims: Option<OidcClaims<GroupClaims>>,
    Extension(state): Extension<AppState>,
    session: Session,
    mut request: Request,
    next: Next,
) -> AxumResult<Response> {
    if let Some(auth) = request.headers().get("Authorization").cloned()
        && let Ok(auth_str) = auth.to_str()
        && let Some(token) = auth_str.strip_prefix("Bearer ")
    {
        return Ok(next.run(request).await);
    }

    Err(AxumError::unauthorized(eyre::eyre!("Unauthorized")))
}
