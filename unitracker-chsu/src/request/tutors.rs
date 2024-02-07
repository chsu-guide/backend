use crate::error::ScheduleError;
use crate::model::tutors::Tutors;

const TUTOR_ENDPOINT: String = "https://www.chsu.ru/raspisanie/cache/tutor.json".into();
pub async fn get_groups() -> Result<Vec<Tutors>, ScheduleError> {
    let res = reqwest::get(TUTOR_ENDPOINT).await?.bytes().await?;
    let res_format = serde_json::from_slice::<Vec<Tutors>>(&res)?;
}