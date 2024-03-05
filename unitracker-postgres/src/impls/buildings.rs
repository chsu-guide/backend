use crate::database::Database;
use eyre::{Result, WrapErr};
use crate::models::buildings::DbBuilding;

impl Database {
    pub async fn select_buildings(&self, id: i64) -> Result<Option<DbBuilding>> {
        let query = sqlx::query_as!(DbBuilding,
        r#"
        SELECT
            id, title
        FROM
            building
        WHERE
            id = $1
        "#,
            id
        );

        query.fetch_optional(self)
            .await
            .wrap_err("Failed to fetch building")
    }
    pub async fn insert_buildings(&self, building: DbBuilding) -> Result<()> {
        let params = building;
        let query = sqlx::query!(r#"
        INSERT INTO building
        (id, title)
        VALUES ($1, $2)
        ON CONFLICT (id) DO NOTHING
            "#,
            params.id,
            &params.title
        );

        query
            .execute(self).await
            .wrap_err("Failed to insert auditoriums")?;
        Ok(())
    }
}