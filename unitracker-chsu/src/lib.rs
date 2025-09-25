use std::sync::OnceLock;

use reqwest::{Client, ClientBuilder, redirect::Policy};

use crate::request::auth::get_auth;

mod error;
pub mod model;
pub mod request;
pub mod utils;
pub static CLIENT: OnceLock<Client> = OnceLock::new();
pub static CONFIG: OnceLock<String> = OnceLock::new();

pub struct ChsuClient {
    inner: Client,
    config: String,
}

impl ChsuClient {
    pub async fn new() -> Self {
        let tmp_client = ClientBuilder::new().build().unwrap();
        let cfg = get_auth(&tmp_client).await.unwrap().data;
        Self {
            inner: Client::new(),
            config: cfg,
        }
    }
}
