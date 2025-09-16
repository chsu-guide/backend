use crate::{get_bearer, get_client, request::RequestErrors};
use reqwest::{Error, Method, Response, StatusCode};
use serde::de::DeserializeOwned;
use std::future::Future;

pub fn call_with_url(url: &str) -> impl Future<Output = Result<Response, Error>> {
    let client = get_client();
    let bearer = get_bearer();
    // let client = ClientBuilder::new().user_agent("").build().unwrap();
    let response = client
        .request(Method::GET, url)
        .header("content-type", "application/json")
        .bearer_auth(bearer)
        .send();
    response
}

pub async fn check_result<T: DeserializeOwned>(response: Response) -> Result<T, RequestErrors> {
    match response.status() {
        StatusCode::OK => Ok(response.json().await?),
        StatusCode::UNAUTHORIZED => Err(RequestErrors::InvalidBearerToken),
        _ => Err(RequestErrors::UnknownError),
    }
}
