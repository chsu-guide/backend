use hyper::StatusCode;
use reqwest::Response;
use serde::Deserialize;

use crate::request::RequestErrors;

pub trait ToConcrete<T> {
    type Concrete;
    fn to_concrete(self) -> impl Future<Output = Result<Self::Concrete, RequestErrors>>;
}

impl<T, U> ToConcrete<T> for U
where
    T: Sized + for<'de> Deserialize<'de>,
    U: Future<Output = Result<Response, reqwest::Error>>,
{
    type Concrete = T;

    async fn to_concrete(self) -> Result<Self::Concrete, RequestErrors> {
        match self.await {
            Ok(resp) if resp.status() == StatusCode::OK => resp
                .json::<Self::Concrete>()
                .await
                .map_err(RequestErrors::ReqwestError),
            Ok(_) => Err(RequestErrors::UnknownError),
            Err(e) => Err(RequestErrors::ReqwestError(e)),
        }
    }
}
