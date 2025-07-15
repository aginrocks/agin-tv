use color_eyre::Result;
use reqwest::Url;
use std::env;

pub fn build_url(endpoint: &str) -> Result<Url> {
    Ok(Url::parse(&env::var("API_URL")?)?.join(&format!("/api{endpoint}"))?)
}
