use dotenv::dotenv;
use std::env;
use serde_derive;
use serde_derive::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct AuthRequest {
    username: String,
    password: String,
}

impl AuthRequest {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

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
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthResponse {
    data: String,
    error: Option<String>,
}