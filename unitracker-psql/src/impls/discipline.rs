use eyre::{Context, Result};

use crate::{database::Database, models::discipline::Discipline};

impl Database {
    /// Select by ID
    pub async fn select_discipline(&self, id: i64) -> Result<Option<Discipline>> {
        let query = sqlx::query_as!(
            Discipline,
            r#"
            SELECT id, name
            FROM discipline
            WHERE id = $1
            "#,
            id
        );
        query
            .fetch_optional(self)
            .await
            .wrap_err("Failed to fetch discipline")
    }
    pub async fn insert_discipline(&self, discipline: &Discipline) -> Result<()> {
        let query = sqlx::query!(
            r#"
            INSERT INTO discipline
            (id, name)
            VALUES ($1, $2)
            ON CONFLICT (id) DO
            NOTHING
            "#,
            discipline.id,
            &discipline.name
        );

        query
            .execute(self)
            .await
            .wrap_err("Failed to insert a discipline")?;
        Ok(())
    }
    pub async fn insert_discipline_many(&self, discipline_list: &[Discipline]) -> Result<()> {
        let (ids, names): (Vec<i64>, Vec<String>) = discipline_list
            .iter()
            .map(|d| (d.id, d.name.to_string()))
            .unzip();
        let query = sqlx::query!(
            r#"
            INSERT INTO discipline
            (id, name)
            SELECT * FROM UNNEST($1::bigint[], $2::text[])
            ON CONFLICT (id) DO NOTHING;
            "#,
            &ids,
            &names
        );
        let _ = query.execute(self).await?;
        Ok(())
    }
}
