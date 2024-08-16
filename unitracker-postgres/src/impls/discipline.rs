use eyre::{Result, WrapErr};
use crate::database::Database;
use crate::models::discipline::DbDiscipline;

impl Database {
    pub async fn insert_discipline() -> Result<()> {
        todo!()
    }

    pub async fn insert_discipline_many() -> Result<()> {
        todo!()
    }

    pub async fn select_discipline() -> Result<Option<DbDiscipline>> {
        todo!()
    }
}