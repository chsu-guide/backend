use crate::ChsuClient;
use crate::model::auditoriums::*;
use crate::request::RequestErrors;
use crate::request::constants::*;
use crate::utils::response::ToConcrete;

impl ChsuClient {
    pub async fn get_auditoriums(&self) -> Result<AuditoriumList, RequestErrors> {
        self.call_with_url(AUDITORIUM_URL).to_concrete().await
    }
}
