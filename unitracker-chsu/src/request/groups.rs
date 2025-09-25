use crate::ChsuClient;
use crate::model::groups::GroupList;
use crate::request::RequestErrors;
use crate::request::constants::STUDENT_GROUP_URL;
use crate::utils::response::ToConcrete;

impl ChsuClient {
    pub async fn get_groups(&self) -> Result<GroupList, RequestErrors> {
        self.call_with_url(STUDENT_GROUP_URL).to_concrete().await
    }
}
