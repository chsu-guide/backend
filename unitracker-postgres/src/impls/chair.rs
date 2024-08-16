use eyre::{Result, WrapErr};
use crate::database::Database;
use crate::models::chair::DbChair;

impl Database {
    pub async fn insert_chair() -> Result<()> {
        todo!()
    }

    pub async fn insert_chair_many() -> Result<()> {
        todo!()
    }

    pub async fn select_chair() -> Result<Option<DbChair>> {
        todo!()
    }
}