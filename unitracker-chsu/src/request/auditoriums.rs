use crate::model::auditoriums::*;
use crate::request::constants::*;
use crate::request::util::{call_with_url, check_result};
use crate::request::RequestErrors;

pub async fn get_auditoriums() -> Result<AuditoriumList, RequestErrors> {
    let auditorium_url = BASE_URL.to_owned() + AUDITORIUM;
    let response = call_with_url(&auditorium_url).await?;
    check_result(response).await
}
