use crate::model::buildings::BuildingList;
use crate::request::constants::*;
use crate::request::util::{call_with_url, check_result};
use crate::request::RequestErrors;

pub async fn get_buildings() -> Result<BuildingList, RequestErrors> {
    let buildings_url = BASE_URL.to_owned() + BUILDING;
    let response = call_with_url(&buildings_url).await?;
    check_result(response).await
}
