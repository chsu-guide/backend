use serde_derive;
use serde_derive::{Deserialize, Serialize};
use std::env;

/// Body of an [`get_auth()`](crate::request::auth::get_auth) request
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthRequest {
    username: String,
    password: String,
}
impl AuthRequest {
    /// Build new [`AuthRequest`] based on login and password in env vars
    pub fn new() -> Self {
        let username = match env::var("LOGIN") {
            Ok(u) => u,
            Err(e) => panic!("username: {}", e),
        };
        let password = match env::var("PASSWORD") {
            Ok(p) => p,
            Err(e) => panic!("password: {}", e),
        };

        Self { username, password }
    }
}
/// Response to an [`get_auth()`](crate::request::auth::get_auth) request
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AuthResponse {
    /// Bearer token
    pub data: String,
    pub error: Option<String>,
}
