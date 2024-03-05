use thiserror::Error;

pub mod auditoriums;
mod constants;
pub mod auth;
pub mod teachers;
pub mod groups;
pub mod buildings;
pub mod schedule;

#[derive(Error, Debug)]
pub enum AuthErrors {
    #[error("Request body is empty")]
    EmptyRequestBody,
    #[error("Login or password are incorrect")]
    IncorrectAuthData,
    #[error("Unknown error")]
    UnknownError,
    #[error("Reqwest error")]
    ReqwestError(#[from] reqwest::Error),
}
#[derive(Error, Debug)]
pub enum RequestErrors {
    #[error("Bearer token is invalid")]
    InvalidBearerToken,
    #[error("Unknown error")]
    UnknownError,
    #[error("Generic reqwest error, your Bearer token is probably invalid")]
    ReqwestError(#[from] reqwest::Error),
}