use axum::{extract::Request, middleware::Next, response::Response};
use color_eyre::eyre::Context;
use http::HeaderValue;

use crate::axum_error::AxumResult;

pub async fn session_bearer_override(mut req: Request, next: Next) -> AxumResult<Response> {
    if let Some(auth) = req.headers().get("Authorization").cloned()
        && let Ok(auth_str) = auth.to_str()
        && let Some(token) = auth_str.strip_prefix("Bearer ")
    {
        let header_value = HeaderValue::from_str(&format!("id={token}")).wrap_err("XD")?;
        if let Some(token) = req.headers_mut().get_mut("Cookie") {
            *token = header_value;
        } else {
            req.headers_mut().insert("Cookie", header_value);
        }
    }

    Ok(next.run(req).await)
}
