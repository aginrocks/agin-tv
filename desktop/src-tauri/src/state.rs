use openidconnect::{
    core::{
        CoreAuthDisplay, CoreAuthPrompt, CoreErrorResponseType, CoreGenderClaim, CoreJsonWebKey,
        CoreJweContentEncryptionAlgorithm, CoreRegisterErrorResponseType,
        CoreTokenIntrospectionResponse, CoreTokenResponse,
    },
    CsrfToken, EmptyAdditionalClaims, PkceCodeChallenge,
};
use reqwest::Client;
use std::{net::SocketAddr, sync::Arc};

// Dont touch it i'm not doing it again in case it will be needed in the future
// pub type OidcClient_old = openidconnect::Client<
//     EmptyAdditionalClaims,
//     CoreAuthDisplay,
//     CoreGenderClaim,
//     CoreJweContentEncryptionAlgorithm,
//     CoreJsonWebKey,
//     CoreAuthPrompt,
//     openidconnect::StandardErrorResponse<CoreErrorResponseType>,
//     CoreTokenResponse,
//     CoreTokenIntrospectionResponse,
//     openidconnect::core::CoreRevocableToken,
//     openidconnect::StandardErrorResponse<CoreRegisterErrorResponseType>,
//     openidconnect::EndpointSet,
//     openidconnect::EndpointNotSet,
//     openidconnect::EndpointNotSet,
//     openidconnect::EndpointNotSet,
//     openidconnect::EndpointSet,
//     openidconnect::EndpointNotSet,
// >;

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
    pub http_client: Client,
    pub csrf_token: CsrfToken,
    pub client: Arc<OidcClient>,
    pub socket_addr: SocketAddr,
    pub pkce: Arc<(PkceCodeChallenge, String)>,
}
