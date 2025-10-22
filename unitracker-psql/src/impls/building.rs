use eyre::{Context, Result};

use crate::{
    database::Database,
    models::building::{Building, BuildingWithAuditoriums},
};

impl Database {
    /// Select a building by its ID
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
    /// Select a building by its name
    /// Names are supposedly unique, returns the first match regardless
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
    pub async fn select_buildings_with_auditoriums(&self) -> Result<Vec<BuildingWithAuditoriums>> {
        let query = sqlx::query_as!(
            Building,
            r#"
            SELECT id, name
            FROM building
            "#
        )
        .fetch_all(self)
        .await
        .wrap_err("Failed to fetch building list")?;

        let mut buildings_with_auditoriums = vec![];
        for building in query {
            let auds = self.select_auditorium_by_building_all(building.id).await?;
            buildings_with_auditoriums.push(BuildingWithAuditoriums {
                id: building.id,
                name: building.name,
                auditoriums: auds,
            });
        }
        Ok(buildings_with_auditoriums)
    }
    /// Insert a building
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
    /// Insert a list of buildings
    /// WARNING: Heavier due to splitting and unnesting
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
