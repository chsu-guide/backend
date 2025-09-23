use crate::model::buildings::BuildingList;
use crate::request::constants::*;
use crate::request::util::check_result;
use crate::request::RequestErrors;
use crate::ChsuClient;

impl ChsuClient {
    pub async fn get_buildings(&self) -> Result<BuildingList, RequestErrors> {
        let buildings_url = BASE_URL.to_owned() + BUILDING;
        let response = self.call_with_url(&buildings_url).await?;
        check_result(response).await
    }
}
