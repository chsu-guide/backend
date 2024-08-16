use eyre::{Result, WrapErr};
use crate::database::Database;
use crate::models::faculty::DbFaculty;

impl Database {
    pub async fn insert_faculty() -> Result<()> {
        todo!()
    }

    pub async fn insert_faculty_many() -> Result<()> {
        todo!()
    }

    pub async fn select_faculty() -> Result<Option<DbFaculty>> {
        todo!()
    }
}