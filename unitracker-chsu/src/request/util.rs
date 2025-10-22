use crate::ChsuClient;
use chrono::{
    NaiveDate,
    format::{DelayedFormat, StrftimeItems},
};
use reqwest::{Error, Method, Request, Response};
use std::future::Future;

impl ChsuClient {
    pub fn call_with_url(&self, url: &str) -> impl Future<Output = Result<Response, Error>> {
        let response = self.build_request(url);
        self.execute(response)
    }

    fn build_request(&self, url: &str) -> Request {
        self.inner
            .request(Method::GET, url)
            .bearer_auth(&self.config)
            .build()
            .unwrap()
    }

    fn execute(&self, request: Request) -> impl Future<Output = Result<Response, Error>> {
        self.inner.execute(request)
    }
}

pub fn format_date<'a>(date: NaiveDate) -> DelayedFormat<StrftimeItems<'a>> {
    date.format("%d.%m.%Y")
}
