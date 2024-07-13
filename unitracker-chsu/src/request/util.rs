use std::future::Future;
use reqwest::{ClientBuilder, Error, Method, Response, StatusCode};
use serde::de::DeserializeOwned;
use crate::request::RequestErrors;

pub fn call_with_url(url: &str, bearer: &str) -> impl Future<Output = Result<Response, Error>> {
    let client = ClientBuilder::new().user_agent("").build();
    let mut response = client
        .unwrap()
        .request(Method::GET, url)
        .header("content-type", "application/json")
        .bearer_auth(bearer)
        .send();
    response
}

pub async fn check_result<T: DeserializeOwned>(response: Response) -> Result<T, RequestErrors>{
    match response.status() {
        StatusCode::OK => Ok(response.json().await?),
        StatusCode::UNAUTHORIZED => Err(RequestErrors::InvalidBearerToken),
        _ => Err(RequestErrors::UnknownError)
    }
}