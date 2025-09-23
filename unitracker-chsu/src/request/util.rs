use crate::{request::RequestErrors, ChsuClient};
use chrono::{
    format::{DelayedFormat, StrftimeItems},
    NaiveDate,
};
use reqwest::{Error, Method, Response, StatusCode};
use serde::de::DeserializeOwned;
use std::future::Future;

impl ChsuClient {
    pub fn call_with_url(&self, url: &str) -> impl Future<Output = Result<Response, Error>> {
        let response = self
            ._inner
            .request(Method::GET, url)
            .header("content-type", "application/json")
            .bearer_auth(&self._config)
            .send();
        response
    }
}

pub async fn check_result<T: DeserializeOwned>(response: Response) -> Result<T, RequestErrors> {
    match response.status() {
        StatusCode::OK => Ok(response.json().await?),
        StatusCode::UNAUTHORIZED => Err(RequestErrors::InvalidBearerToken),
        _ => Err(RequestErrors::UnknownError),
    }
}

pub fn format_date<'a>(date: NaiveDate) -> DelayedFormat<StrftimeItems<'a>> {
    date.format("%d.%m.%Y")
}
