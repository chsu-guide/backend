use crate::database::Database;
use eyre::{Result, WrapErr};
use crate::models::teachers::DbTeacher;

impl Database {
    pub async fn select_teacher(&self, id: i64) -> Result<Option<DbTeacher>> {
        let query = sqlx::query_as!(DbTeacher,
        r#"
        SELECT
            id, last_name, first_name, middle_name
        FROM
            teacher
        WHERE
            id = $1
        "#,
            id
        );

        query.fetch_optional(self)
            .await
            .wrap_err("Failed to fetch building")
    }
    pub async fn insert_teachers(&self, db_teacher: DbTeacher) -> Result<()> {
        let params = db_teacher;
        let query = sqlx::query!(r#"
        INSERT INTO teacher
        (id, last_name, first_name, middle_name)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (id) DO NOTHING
            "#,
            params.id,
            &params.last_name,
            &params.first_name,
            &params.middle_name.unwrap()
        );

        query
            .execute(self).await
            .wrap_err("Failed to insert auditoriums")?;
        Ok(())
    }
}