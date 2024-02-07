use crate::error::ScheduleError;
use crate::model::students::Student;

const STUDENT_ENDPOINT: &str = "https://www.chsu.ru/raspisanie/cache/student.json";
pub async fn get_students() -> Result<Vec<Student>, ScheduleError> {
    let res = reqwest::get(STUDENT_ENDPOINT).await?.text().await?;
    dbg!(&res);
    let res_format = serde_json::from_str::<Vec<Student>>(&res)?;
    Ok(res_format)
}