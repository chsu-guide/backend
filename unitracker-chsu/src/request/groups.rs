use crate::model::groups::GroupList;
use crate::request::constants::{BASE_URL, STUDENT_GROUP};
use crate::request::util::{call_with_url, check_result};
use crate::request::RequestErrors;

pub async fn get_groups() -> Result<GroupList, RequestErrors> {
    let group_url = BASE_URL.to_owned() + STUDENT_GROUP;
    let response = call_with_url(&group_url).await?;
    check_result(response).await
}
