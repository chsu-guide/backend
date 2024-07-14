use reqwest::{Client, ClientBuilder, Method, StatusCode};
use crate::model::groups::GroupList;
use crate::request::{AuthErrors, RequestErrors};
use crate::request::constants::{BASE_URL, STUDENT_GROUP};
use crate::request::util::{call_with_url, check_result};

pub async fn get_groups(client: &mut Client, bearer: &str) -> Result<GroupList, RequestErrors> {
    let group_url = BASE_URL.to_owned() + STUDENT_GROUP;
    let response = call_with_url(client, &group_url, bearer).await?;
    check_result(response).await
}