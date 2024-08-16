use eyre::{Result, WrapErr};
use crate::database::Database;
use crate::models::schedule::DbSchedule;

impl Database {
    pub async fn insert_schedule() -> Result<()> {
        todo!()
    }

    pub async fn insert_schedule_many() -> Result<()> {
        todo!()
    }

    pub async fn select_schedule() -> Result<Option<DbSchedule>> {
        todo!()
    }
}