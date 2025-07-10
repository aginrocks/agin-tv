use openidconnect::{
    core::{
        CoreAuthDisplay, CoreAuthPrompt, CoreErrorResponseType, CoreGenderClaim, CoreJsonWebKey,
        CoreJweContentEncryptionAlgorithm, CoreRegisterErrorResponseType,
        CoreTokenIntrospectionResponse, CoreTokenResponse,
    },
    EmptyAdditionalClaims,
};
use std::{net::SocketAddr, sync::Arc};

type OidcClient = openidconnect::Client<
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
    openidconnect::EndpointSet,
    openidconnect::EndpointNotSet,
>;
#[derive(Clone)]
pub struct AppState {
    pub client: Arc<OidcClient>,
    pub socket_addr: SocketAddr,
}
