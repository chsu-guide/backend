use crate::error::ScheduleError;
use crate::model::students::Students;

const STUDENT_ENDPOINT: String = "https://www.chsu.ru/raspisanie/cache/student.json".into();
pub async fn get_groups() -> Result<Vec<Students>, ScheduleError> {
    let res = reqwest::get(STUDENT_ENDPOINT).await?.bytes().await?;
    let res_format = serde_json::from_slice::<Vec<Students>>(&res)?;
}