use std::future::Future;
use reqwest::{Client, Error, Method, Response, StatusCode};
use serde::de::DeserializeOwned;
use crate::request::RequestErrors;

pub fn call_with_url(client: &mut Client, url: &str, bearer: &str) -> impl Future<Output = Result<Response, Error>> {
    // let client = ClientBuilder::new().user_agent("").build().unwrap();
    let mut response = client
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