use reqwest::{ClientBuilder, Method, StatusCode};
use crate::model::groups::GroupList;
use crate::request::{AuthErrors, RequestErrors};
use crate::request::constants::{BASE_URL, STUDENT_GROUP};

pub async fn get_groups(bearer: &str) -> Result<GroupList, RequestErrors> {
    let teachers_url = BASE_URL.to_owned() + STUDENT_GROUP;
    let client = ClientBuilder::new().user_agent("").build()?;
    let mut response = client
        .request(Method::GET, teachers_url)
        .header("content-type", "application/json")
        .bearer_auth(bearer)
        .send()
        .await?;
    let response_json = match response.status() {
        StatusCode::OK => Ok(response.json().await?),
        StatusCode::UNAUTHORIZED => Err(RequestErrors::InvalidBearerToken),
        _ => Err(RequestErrors::UnknownError)
    };
    response_json
}