use crate::{
    ChsuClient,
    model::disciplines::Discipline,
    request::{RequestErrors, constants::DISCIPLINE_URL},
    utils::response::ToConcrete,
};

impl ChsuClient {
    pub async fn get_disciplines(&self) -> Result<Vec<Discipline>, RequestErrors> {
        self.call_with_url(DISCIPLINE_URL).to_concrete().await
    }
}
