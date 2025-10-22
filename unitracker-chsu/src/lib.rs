use std::sync::OnceLock;

use reqwest::{Client, ClientBuilder};
use tracing::trace;

use crate::request::auth::get_auth;

mod error;
pub mod model;
pub mod request;
pub mod utils;
pub static CLIENT: OnceLock<Client> = OnceLock::new();
pub static CONFIG: OnceLock<String> = OnceLock::new();

#[derive(Debug)]
pub struct ChsuClient {
    inner: Client,
    config: String,
}

impl ChsuClient {
    #[tracing::instrument]
    pub async fn new() -> Self {
        let tmp_client = ClientBuilder::new().build().unwrap();
        trace!("Initialized CHSU API base client");
        let cfg = get_auth(&tmp_client).await.unwrap().data;
        trace!("Got authentication");
        Self {
            inner: Client::new(),
            config: cfg,
        }
    }
}
