use eyre::{Context, Result};

use crate::{database::Database, models::building::Building};

impl Database {
    #[tracing::instrument]
    pub async fn select_building(&self, id: i64) -> Result<Option<Building>> {
        let query = sqlx::query_as!(
            Building,
            r#"
            SELECT id, name
            FROM building
            WHERE id = $1
            "#,
            id
        );
        query
            .fetch_optional(self)
            .await
            .wrap_err("Failed to fetch building")
    }
    #[tracing::instrument]
    pub async fn select_building_by_name(&self, name: &str) -> Result<Option<Building>> {
        let query = sqlx::query_as!(
            Building,
            r#"
            SELECT id, name
            FROM building
            WHERE name ~ $1
            "#,
            name
        );
        query
            .fetch_optional(self)
            .await
            .wrap_err("Failed to fetch a building by name")
    }
    #[tracing::instrument]
    pub async fn insert_building(&self, building: Building) -> Result<()> {
        let query = sqlx::query!(
            r#"
            INSERT INTO building
            (id, name)
            VALUES ($1, $2)
            ON CONFLICT (id) DO
            UPDATE SET
            id = $1,
            name = $2
            "#,
            building.id,
            &building.name
        );

        query
            .execute(self)
            .await
            .wrap_err("Failed to insert a building")?;
        Ok(())
    }
    #[tracing::instrument]
    pub async fn insert_building_many(&self, building_list: &[Building]) -> Result<()> {
        let params = building_list;
        let ids: Vec<i64> = params.iter().map(|au| au.id).collect();
        let names: Vec<String> = params.iter().map(|au| au.name.clone().into()).collect();
        let query = sqlx::query!(
            r#"
            INSERT INTO building
            (id, name)
            VALUES
            (UNNEST($1::BIGINT[]),
            UNNEST($2::TEXT[]))
            "#,
            &ids,
            &names,
        );

        query
            .execute(self)
            .await
            .wrap_err("Failed to insert multiple buildings")?;
        Ok(())
    }
}
