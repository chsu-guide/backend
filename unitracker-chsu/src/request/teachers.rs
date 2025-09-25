use crate::ChsuClient;
use crate::model::teachers::TeacherList;
use crate::request::RequestErrors;
use crate::request::constants::*;
use crate::utils::response::ToConcrete;

/// Get a [`list of all Teachers`](crate::model::teachers::TeacherList) in the university
impl ChsuClient {
    pub async fn get_teachers(&self) -> Result<TeacherList, RequestErrors> {
        self.call_with_url(TEACHERS_URL).to_concrete().await
    }
}
