use eyre::{Result, WrapErr};
use crate::database::Database;
use crate::models::teacher::DbTeacher;

impl Database {
    pub async fn insert_teacher() -> Result<()> {
        todo!()
    }

    pub async fn insert_teacher_many() -> Result<()> {
        todo!()
    }

    pub async fn select_teacher() -> Result<Option<DbTeacher>> {
        todo!()
    }
}