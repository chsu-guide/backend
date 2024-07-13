use reqwest::{ClientBuilder, Method, StatusCode};
use crate::model::buildings::BuildingList;
use crate::request::RequestErrors;
use crate::request::constants::*;
use crate::request::util::{call_with_url, check_result};

pub async fn get_buildings(bearer: &str) -> Result<BuildingList, RequestErrors> {
    let buildings_url = BASE_URL.to_owned() + BUILDING;
    let response = call_with_url(&buildings_url, bearer).await?;
    check_result(response).await
}