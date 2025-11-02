use std::sync::Arc;

use crate::settings::Settings;
use fred::prelude::Pool;
use mongodb::Database;
use openidconnect::{
    CsrfToken, EmptyAdditionalClaims, PkceCodeChallenge,
    core::{
        CoreAuthDisplay, CoreAuthPrompt, CoreErrorResponseType, CoreGenderClaim, CoreJsonWebKey,
        CoreJweContentEncryptionAlgorithm, CoreRegisterErrorResponseType,
        CoreTokenIntrospectionResponse, CoreTokenResponse,
    },
};

pub type OidcClient = openidconnect::Client<
    EmptyAdditionalClaims,
    CoreAuthDisplay,
    CoreGenderClaim,
    CoreJweContentEncryptionAlgorithm,
    CoreJsonWebKey,
    CoreAuthPrompt,
    openidconnect::StandardErrorResponse<CoreErrorResponseType>,
    CoreTokenResponse,
    CoreTokenIntrospectionResponse,
    openidconnect::core::CoreRevocableToken,
    openidconnect::StandardErrorResponse<CoreRegisterErrorResponseType>,
    openidconnect::EndpointSet,
    openidconnect::EndpointNotSet,
    openidconnect::EndpointNotSet,
    openidconnect::EndpointNotSet,
    openidconnect::EndpointMaybeSet,
    openidconnect::EndpointMaybeSet,
>;

#[derive(Clone)]
pub struct AppState {
    pub http_client: reqwest::Client,
    pub db: Database,
    pub settings: Arc<Settings>,
    pub oidc_client: Arc<OidcClient>,
    pub redis_store: Pool,
}
