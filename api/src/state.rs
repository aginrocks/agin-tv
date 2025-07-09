use std::sync::Arc;

use crate::settings::Settings;
use mongodb::Database;

#[derive(Clone)]
pub struct AppState {
    pub http_client: reqwest::Client,
    pub db: Database,
    pub settings: Arc<Settings>,
}
