use crate::model::auth::*;
use crate::request::AuthErrors;
use crate::request::constants::AUTH_SIGNIN_URL;
use reqwest::{Body, Client, Method, StatusCode};
use serde_json;

pub async fn get_auth(client: &Client) -> Result<AuthResponse, AuthErrors> {
    let auth_request = AuthRequest::new();
    let auth_request_de: Body = match serde_json::to_string(&auth_request) {
        Ok(auth) => auth.into(),
        Err(e) => panic!("{}", e),
    };
    let response = client
        .request(Method::POST, AUTH_SIGNIN_URL)
        .header("content-type", "application/json")
        .body(auth_request_de);
    let response = response.send().await?;

    let auth_response = match response.status() {
        StatusCode::OK => Ok(response.json().await?),
        StatusCode::BAD_REQUEST => Err(AuthErrors::EmptyRequestBody),
        StatusCode::UNAUTHORIZED => Err(AuthErrors::IncorrectAuthData),
        _ => Err(AuthErrors::UnknownError),
    };
    auth_response
}
