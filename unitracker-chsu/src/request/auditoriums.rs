use crate::model::auditoriums::*;
use crate::request::constants::*;
use crate::request::util::check_result;
use crate::request::RequestErrors;
use crate::ChsuClient;

impl ChsuClient {
    pub async fn get_auditoriums(&self) -> Result<AuditoriumList, RequestErrors> {
        let auditorium_url = BASE_URL.to_owned() + AUDITORIUM;
        let response = self.call_with_url(&auditorium_url).await?;
        check_result(response).await
    }
}
