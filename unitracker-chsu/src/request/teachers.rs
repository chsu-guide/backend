use crate::model::teachers::TeacherList;
use crate::request::constants::*;
use crate::request::util::{call_with_url, check_result};
use crate::request::RequestErrors;

/// Get a [`list of all Teachers`](crate::model::teachers::TeacherList) in the university
pub async fn get_teachers() -> Result<TeacherList, RequestErrors> {
    let teachers_url = BASE_URL.to_owned() + TEACHERS;
    let response = call_with_url(&teachers_url).await?;
    check_result(response).await
}
