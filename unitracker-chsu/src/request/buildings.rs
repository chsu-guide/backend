use crate::ChsuClient;
use crate::model::buildings::BuildingList;
use crate::request::RequestErrors;
use crate::request::constants::*;
use crate::utils::response::ToConcrete;

impl ChsuClient {
    pub async fn get_buildings(&self) -> Result<BuildingList, RequestErrors> {
        self.call_with_url(BUILDING_URL).to_concrete().await
    }
}
