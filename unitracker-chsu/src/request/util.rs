use crate::ChsuClient;
use chrono::{
    NaiveDate,
    format::{DelayedFormat, StrftimeItems},
};
use reqwest::{Error, Method, Response};
use std::future::Future;

impl ChsuClient {
    pub fn call_with_url(&self, url: &str) -> impl Future<Output = Result<Response, Error>> {
        let response = self
            .inner
            .request(Method::GET, url)
            .header("authorization", format!("Bearer {}", self.config))
            .build()
            .unwrap();
        self.inner.execute(response)
    }
}

pub fn format_date<'a>(date: NaiveDate) -> DelayedFormat<StrftimeItems<'a>> {
    date.format("%d.%m.%Y")
}
