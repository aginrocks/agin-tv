use reqwest::Client;
use std::{net::SocketAddr, sync::Arc};

#[derive(Clone)]
pub struct AppState {
    pub http_client: Client,
    pub socket_addr: SocketAddr,
    pub cookie_store: Arc<reqwest::cookie::Jar>,
}
