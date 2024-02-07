use crate::error::ScheduleError;
use crate::model::tutors::Tutor;

const TUTOR_ENDPOINT: &str = "https://www.chsu.ru/raspisanie/cache/tutor.json?";
pub async fn get_tutors() -> Result<Vec<Tutor>, ScheduleError> {
    let res = reqwest::get(TUTOR_ENDPOINT).await?.text().await?;
    dbg!(&res);
    let res_format = serde_json::from_str::<Vec<Tutor>>(&res)?;
    Ok(res_format)
}