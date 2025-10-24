use eyre::{Context, Result};
use itertools::Itertools;

use crate::{database::Database, models::discipline::Discipline};

impl Database {
    /// Select by ID
    #[tracing::instrument]
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

    #[tracing::instrument]
    pub async fn select_discipline_all(&self) -> Result<Vec<Discipline>> {
        let query = sqlx::query_as!(
            Discipline,
            r#"
            SELECT id, name
            FROM discipline
            "#,
        );
        query
            .fetch_all(self)
            .await
            .wrap_err("Failed to fetch discipline")
    }

    #[tracing::instrument]
    pub async fn insert_discipline(&self, discipline: &Discipline) -> Result<()> {
        let query = sqlx::query!(
            r#"
            INSERT INTO discipline
            (name)
            VALUES ($1)
            ON CONFLICT (name) DO
            NOTHING
            "#,
            &discipline.name
        );

        query
            .execute(self)
            .await
            .wrap_err("Failed to insert a discipline")?;
        Ok(())
    }
    #[tracing::instrument]
    pub async fn insert_discipline_many(&self, discipline_list: &[Discipline]) -> Result<()> {
        let names: Vec<String> = discipline_list
            .iter()
            .map(|d| d.name.to_string())
            .sorted()
            .dedup()
            .collect();
        let query = sqlx::query!(
            r#"
            INSERT INTO discipline
            (name)
            SELECT * FROM UNNEST($1::TEXT[])
            ON CONFLICT (name) DO NOTHING;
            "#,
            &names
        );
        let _ = query.execute(self).await?;
        Ok(())
    }
}
