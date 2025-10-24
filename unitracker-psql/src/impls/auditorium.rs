use std::collections::HashMap;

use crate::{
    database::Database,
    models::{auditorium::Auditorium, class::Class},
};

use chrono::NaiveDateTime;
use eyre::{Context, Result};
use unitracker_types::IdOrName;
impl Database {
    /// Select an auditorium by its ID
    #[tracing::instrument]
    pub async fn select_auditorium(&self, id: i64) -> Result<Option<Auditorium>> {
        let query = sqlx::query_as!(
            Auditorium,
            r#"
            SELECT
                id, name, number, building_id
            FROM auditorium
            WHERE id = $1
            "#,
            id
        );
        query
            .fetch_optional(self)
            .await
            .wrap_err("Failed to fetch auditorium")
    }

    pub async fn select_auditorium_by_building_all(&self, id: i64) -> Result<Vec<Auditorium>> {
        let query = sqlx::query_as!(
            Auditorium,
            r#"
            SELECT
                auditorium.id, auditorium.name, auditorium.number, building_id
            FROM auditorium
            INNER JOIN building ON auditorium.building_id = building.id
            WHERE building_id = $1
            "#,
            id
        );
        query
            .fetch_all(self)
            .await
            .wrap_err("Failed to fetch auditoriums by building")
    }

    /// Select an auditorium by its name
    pub async fn select_auditorium_by_name(&self, name: &str) -> Result<Option<Auditorium>> {
        let query = sqlx::query_as!(
            Auditorium,
            r#"
             SELECT
                id, name, number, building_id
            FROM auditorium
            WHERE name ~ $1
            "#,
            name
        );
        query
            .fetch_optional(self)
            .await
            .wrap_err("Failed to fetch Auditorium with query")
    }

    pub async fn auditorium_is_available(
        &self,
        name: IdOrName,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Result<Vec<Class>> {
        match name {
            IdOrName::Id(id) => self.auditorium_is_available_by_id(id, start, end).await,
            IdOrName::Name(name) => {
                self.auditorium_is_available_by_name(&name, start, end)
                    .await
            }
        }
    }

    /// Select auditorium by name and check its availability based on the schedule
    /// Return a list of classes in the auditorium during the time range
    async fn auditorium_is_available_by_name(
        &self,
        name: &str,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Result<Vec<crate::models::class::Class>> {
        let query = sqlx::query_as!(
            crate::models::class::Class,
            r#"
            SELECT schedule.id, request_date AS created_at, start_time, end_time, lesson_type, lesson_type_abbr AS lesson_type_abbreviated, discipline_id
            FROM schedule
            JOIN schedule_auditorium ON schedule_auditorium.schedule_id = schedule.id
            JOIN auditorium ON auditorium.id = schedule_auditorium.auditorium_id
            WHERE schedule.start_time >= $1 AND schedule.end_time <= $2 AND auditorium.name = $3
            "#,
            start,
            end,
            name,
        );
        query
            .fetch_all(self)
            .await
            .wrap_err("Failed to fetch classes")
    }

    /// Select auditorium by id and check its availability based on the schedule
    /// Return a list of classes in the auditorium during the time range
    async fn auditorium_is_available_by_id(
        &self,
        name: i64,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Result<Vec<crate::models::class::Class>> {
        let query = sqlx::query_as!(
            crate::models::class::Class,
            r#"
            SELECT schedule.id, request_date AS created_at, start_time, end_time, lesson_type, lesson_type_abbr AS lesson_type_abbreviated, discipline_id
            FROM schedule
            JOIN schedule_auditorium ON schedule_auditorium.schedule_id = schedule.id
            JOIN auditorium ON auditorium.id = schedule_auditorium.auditorium_id
            WHERE schedule.start_time >= $1 AND schedule.end_time <= $2 AND auditorium.id= $3
            "#,
            start,
            end,
            name,
        );
        query
            .fetch_all(self)
            .await
            .wrap_err("Failed to fetch classes")
    }
    /// Insert an Auditorium
    pub async fn insert_auditorium(&self, auditorium: Auditorium) -> Result<()> {
        let params = auditorium;
        let query = sqlx::query!(
            r#"
            INSERT INTO auditorium
            (id, name, number, building_id)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (id) DO UPDATE
            SET
                name = $2,
                number = $3,
                building_id = $4
            "#,
            params.id,
            &params.name,
            &params.number,
            params.building_id
        );
        query
            .execute(self)
            .await
            .wrap_err("Failed to insert one auditorium")?;
        Ok(())
    }
    /// Insert list of auditoriums
    /// WARNING: Very memory heavy compared to other insertions
    pub async fn insert_auditorium_many(
        &self,
        auditorium_list: &[unitracker_chsu::model::auditoriums::Auditorium],
    ) -> Result<()> {
        let params = auditorium_list;
        let db_buildings: HashMap<String, i64> = self
            .select_building_all()
            .await?
            .into_iter()
            .map(|au| (au.name.into_string(), au.id))
            .collect();

        println!("Buildings: {db_buildings:#?}");

        let trans = self.begin().await?;

        for auditorium in params {
            let name = &auditorium.name;
            let number = &auditorium.number;
            let building = db_buildings.get(&auditorium.building_name);
            let building_id = match building {
                Some(id) => *id,
                None => self
                    .insert_building(&auditorium.building_name)
                    .await
                    .unwrap(),
            };
            let _ = sqlx::query!(
                r#"
                INSERT INTO auditorium
                (name, number, building_id)
                VALUES
                ($1, $2, $3)
                "#,
                &name,
                &number,
                building_id
            )
            .execute(self)
            .await
            .wrap_err("Failed to insert building")?;
        }

        trans.commit().await?;
        Ok(())
    }
}
