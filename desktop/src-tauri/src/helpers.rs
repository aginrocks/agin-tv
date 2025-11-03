use reqwest::Url;
use std::env;

pub fn build_url(endpoint: &str) -> Result<Url, String> {
    // let api_url = env::var("API_URL").map_err(|e| e.to_string())?;
    let api_url = "https://tvapi.agin.rocks/api/".to_string();
    let base_url = Url::parse(&api_url).map_err(|e| e.to_string())?;
    let full_url = base_url
        .join(&format!("/api{endpoint}"))
        .map_err(|e| e.to_string())?;
    Ok(full_url)
}
