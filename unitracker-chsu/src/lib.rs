use std::sync::OnceLock;

use dotenv::dotenv;
use reqwest::{Client, ClientBuilder};

use crate::request::auth::get_auth;

mod error;
pub mod model;
pub mod request;
pub static CLIENT: OnceLock<Client> = OnceLock::new();
pub static CONFIG: OnceLock<String> = OnceLock::new();

pub async fn global_init() {
    dotenv().unwrap();

    CLIENT
        .set(ClientBuilder::new().user_agent("Maomao").build().unwrap())
        .unwrap();

    CONFIG.set(get_auth().await.unwrap().data).unwrap();
}

pub fn get_client() -> &'static Client {
    CLIENT.get().unwrap()
}

pub fn get_bearer() -> &'static String {
    CONFIG.get().unwrap()
}
