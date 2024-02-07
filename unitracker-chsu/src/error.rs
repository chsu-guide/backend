use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScheduleError {
    #[error("Error while parsing url: {0}")]
    ParseError(#[from] url::ParseError),
    #[error("Error during http request: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Error during deserialization: {0}")]
    SerdeError(#[from] serde_json::Error),
}