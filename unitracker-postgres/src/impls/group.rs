use eyre::{Result, WrapErr};
use crate::database::Database;
use crate::models::group::DbGroup;

impl Database {
    pub async fn insert_group() -> Result<()> {
        todo!()
    }

    pub async fn insert_group_many() -> Result<()> {
        todo!()
    }

    pub async fn select_group() -> Result<Option<DbGroup>> {
        todo!()
    }
}