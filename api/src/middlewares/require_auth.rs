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
    let token = session
        .get::<String>("access_token")
        .await
        .map_err(|_| AxumError::unauthorized(eyre::eyre!("Unauthorized")))?;

    if token.is_none() {
        return Err(AxumError::unauthorized(eyre::eyre!("Unauthorized")));
    }

    // let claims = claims.ok_or_else(|| AxumError::unauthorized(eyre::eyre!("Unauthorized")))?;

    // let sub = claims.subject().to_string();
    // let name = claims
    //     .name()
    //     .wrap_err("Name is required")?
    //     .get(None)
    //     .wrap_err("Name is required")?
    //     .to_string();
    // let email = claims.email().wrap_err("Email is required")?.to_string();

    // let user = state
    //     .db
    //     .collection::<User>("users")
    //     .find_one_and_update(
    //         doc! { "sub": &sub },
    //         doc! {
    //             "$set": {
    //                 "subject": sub,
    //                 "name": name,
    //                 "email": email,
    //             }
    //         },
    //     )
    //     .upsert(true)
    //     .return_document(ReturnDocument::After)
    //     .await?
    //     .wrap_err("User not found (wtf?")?;

    // request.extensions_mut().insert(UserData(user.clone()));
    // request.extensions_mut().insert(UserId(user.id));

    Ok(next.run(request).await)
}
