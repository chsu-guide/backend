use std::sync::OnceLock;

use dotenv::dotenv;
use hyper::header::GetAll;
use reqwest::{Client, ClientBuilder};

use crate::request::auth::get_auth;

mod error;
pub mod model;
pub mod request;
pub static CLIENT: OnceLock<Client> = OnceLock::new();
pub static CONFIG: OnceLock<String> = OnceLock::new();

pub struct ChsuClient {
    _inner: Client,
    _config: String,
}

impl ChsuClient {
    pub async fn new() -> Self {
        let tmp_client = ClientBuilder::new()
            .user_agent("Unitracker")
            .build()
            .unwrap();
        let cfg = get_auth(&tmp_client).await.unwrap().data;
        Self {
            _inner: Client::new(),
            _config: String::new(),
        }
    }
}
