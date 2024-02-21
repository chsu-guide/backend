use serde::Serialize;
use serde_json;
use crate::model::auth::*;
use reqwest::{Body, Client, ClientBuilder, Error, Method, RequestBuilder, Response};
use crate::request::constants::{AUTH_SIGNIN, BASE_URL};

pub async fn get_auth() -> AuthResponse {
    let auth_request = AuthRequest::new();
    let auth_request_de = match serde_json::to_string(&auth_request) {
        Ok(auth) => auth,
        Err(e) => panic!("{}", e),
    };
    let auth_url = BASE_URL.to_owned() + AUTH_SIGNIN;
    let request_body: Body = auth_request_de.into();
    let user_agent = "balls";

    let client = ClientBuilder::new().user_agent(user_agent).build().unwrap();
    let response = match client
        .request(Method::GET, auth_url)
        .body(request_body)
        .send()
        .await {
        Ok(res) => res,
        Err(e) => panic!("{}", e),
    };

    let auth_data: AuthResponse = response.json().await.unwrap();
    auth_data
}