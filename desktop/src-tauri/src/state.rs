use reqwest::Client;
use std::{io::Error, net::SocketAddr, sync::Arc};
use tokio::task::{AbortHandle, JoinHandle};

#[derive(Clone)]
pub struct AppState {
    pub http_client: Client,
    pub socket_addr: SocketAddr,
    pub cookie_store: Arc<reqwest::cookie::Jar>,
    pub abort_handle: Arc<tokio::sync::RwLock<Option<AbortHandle>>>,
}
