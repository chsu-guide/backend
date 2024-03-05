use crate::database::Database;
use eyre::{Result, WrapErr};
use crate::models::auditoriums::DbAuditorium;

impl Database {
    pub async fn select_auditoriums(&self, id: i64) -> Result<Option<DbAuditorium>> {
        let query = sqlx::query_as!(DbAuditorium,
        r#"
        SELECT
            id, name, number, height, width, length, building_id
        FROM
            auditorium
        WHERE
            id = $1
        "#,
            id
        );

        query.fetch_optional(self)
            .await
            .wrap_err("Failed to fetch auditoriums")
    }
    pub async fn insert_auditoriums(&self, auditorium: DbAuditorium) -> Result<()> {
        let params = auditorium;
        let query = sqlx::query!(r#"
        INSERT INTO auditorium
        (id, name, number, height, width, length, building_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (id) DO
        UPDATE
        SET
            name = $2,
            number = $3,
            height = $4,
            width = $5,
            length = $6,
            building_id = $7
            "#,
            params.id,
            &params.name,
            &params.number,
            params.height,
            params.width,
            params.length,
            params.building_id
        );

        query
            .execute(self).await
            .wrap_err("Failed to insert auditoriums")?;
        Ok(())
    }
}