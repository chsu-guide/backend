use reqwest::{ClientBuilder, Method, StatusCode};
use crate::model::buildings::BuildingList;
use crate::request::RequestErrors;
use crate::request::constants::*;

pub async fn get_buildings(bearer: &str) -> Result<BuildingList, RequestErrors> {
    let teachers_url = BASE_URL.to_owned() + BUILDING;
    let client = ClientBuilder::new().user_agent("").build()?;
    let mut response = client
        .request(Method::GET, teachers_url)
        .header("content-type", "application/json")
        .bearer_auth(bearer)
        .send()
        .await?;
    let response_result = match response.status() {
        StatusCode::OK => Ok(response.json().await?),
        StatusCode::UNAUTHORIZED => Err(RequestErrors::InvalidBearerToken),
        _ => Err(RequestErrors::UnknownError)
    };
    // dbg!(&response_result);
    response_result
}