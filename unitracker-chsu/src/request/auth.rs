use serde_json;
use crate::model::auth::*;
use reqwest::{Body, ClientBuilder, Method, StatusCode};
use crate::request::AuthErrors;
use crate::request::constants::{AUTH_SIGNIN, BASE_URL};



pub async fn get_auth() -> Result<AuthResponse, AuthErrors> {
    let auth_request = AuthRequest::new();
    let auth_request_de: Body = match serde_json::to_string(&auth_request) {
        Ok(auth) => auth.into(),
        Err(e) => panic!("{}", e),
    };
    let auth_url = BASE_URL.to_owned() + AUTH_SIGNIN;
    let client = ClientBuilder::new().user_agent("").build()?;
    let response = client
        .request(Method::POST, auth_url)
        .header("content-type", "application/json")
        .body(auth_request_de)
        .send()
        .await?;

    let auth_response = match response.status() {
        StatusCode::OK => Ok(response.json().await?),
        StatusCode::BAD_REQUEST => Err(AuthErrors::EmptyRequestBody),
        StatusCode::UNAUTHORIZED => Err(AuthErrors::IncorrectAuthData),
        _ => Err(AuthErrors::UnknownError)
    };
    auth_response
}