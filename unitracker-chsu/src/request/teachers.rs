use reqwest::{Client};
use crate::model::teachers::TeacherList;
use reqwest::Error as ReqwestErrorType;
use crate::request::RequestErrors;
use crate::request::constants::*;
use crate::request::util::{call_with_url, check_result};

/// Get a [`list of all Teachers`](crate::model::teachers::TeacherList) in the university
pub async fn get_teachers(client: &mut Client, bearer: &str) -> Result<TeacherList, RequestErrors> {
    let teachers_url = BASE_URL.to_owned() + TEACHERS;
    let response = call_with_url(client, &teachers_url, bearer).await?;
    check_result(response).await
}