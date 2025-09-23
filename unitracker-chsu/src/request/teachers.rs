use crate::model::teachers::TeacherList;
use crate::request::constants::*;
use crate::request::util::check_result;
use crate::request::RequestErrors;
use crate::ChsuClient;

/// Get a [`list of all Teachers`](crate::model::teachers::TeacherList) in the university
impl ChsuClient {
    pub async fn get_teachers(&self) -> Result<TeacherList, RequestErrors> {
        let teachers_url = BASE_URL.to_owned() + TEACHERS;
        let response = self.call_with_url(&teachers_url).await?;
        check_result(response).await
    }
}
