use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde_json::json;

#[derive(Clone)]
pub struct AuthConfig {
    pub username: Option<String>,
    pub password: Option<String>,
}

impl AuthConfig {
    pub fn is_enabled(&self) -> bool {
        self.username.is_some() && self.password.is_some()
    }
}

pub async fn basic_auth_middleware(
    State(auth): State<AuthConfig>,
    req: Request,
    next: Next,
) -> Response {
    // Auth disabled — pass through
    if !auth.is_enabled() {
        return next.run(req).await;
    }

    // Always allow health check without auth
    if req.uri().path() == "/health" {
        return next.run(req).await;
    }

    let expected_user = auth.username.as_deref().unwrap();
    let expected_pass = auth.password.as_deref().unwrap();

    if check_basic_auth(req.headers(), expected_user, expected_pass) {
        next.run(req).await
    } else {
        unauthorized_response()
    }
}

fn check_basic_auth(headers: &HeaderMap, expected_user: &str, expected_pass: &str) -> bool {
    let header_value = match headers.get("authorization").and_then(|v| v.to_str().ok()) {
        Some(v) => v,
        None => return false,
    };

    let encoded = match header_value.strip_prefix("Basic ") {
        Some(e) => e,
        None => return false,
    };

    let decoded_bytes = match STANDARD.decode(encoded) {
        Ok(b) => b,
        Err(_) => return false,
    };

    let decoded = match String::from_utf8(decoded_bytes) {
        Ok(s) => s,
        Err(_) => return false,
    };

    let (user, pass) = match decoded.split_once(':') {
        Some(pair) => pair,
        None => return false,
    };

    // Constant-time comparison to prevent timing attacks
    subtle_comparison(user, expected_user) && subtle_comparison(pass, expected_pass)
}

/// Constant-time string comparison to prevent timing attacks.
fn subtle_comparison(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.bytes()
        .zip(b.bytes())
        .fold(0u8, |acc, (x, y)| acc | (x ^ y))
        == 0
}

fn unauthorized_response() -> Response {
    let headers = [(
        "WWW-Authenticate",
        "Basic realm=\"Home Inventory\"",
    )];
    (
        StatusCode::UNAUTHORIZED,
        headers,
        Json(json!({"error": "Invalid or missing credentials"})),
    )
        .into_response()
}
