use crate::{database::Database, models::auditorium::Auditorium};

use chrono::NaiveDateTime;
use eyre::{Context, Result};
impl Database {
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

    /// Select auditorium by name and check its availability based on the schedule
    /// Return a list of classes in the auditorium during the time range
    pub async fn auditorium_is_available(
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
    /// WARNING: Very memory heavy compared to other insertions
    pub async fn insert_auditorium_many(&self, auditorium_list: &[Auditorium]) -> Result<()> {
        let params = auditorium_list;
        let ids: Vec<i64> = params.iter().map(|au| au.id).collect();
        let names: Vec<String> = params.iter().map(|au| au.name.clone().into()).collect();
        let numbers: Vec<String> = params.iter().map(|au| au.number.clone().into()).collect();
        let buildings: Vec<i64> = params
            .iter()
            .map(|au| au.building_id.unwrap_or_default())
            .collect();
        let query = sqlx::query!(
            r#"
            INSERT INTO auditorium
            (id, name, number, building_id)
            VALUES
            (UNNEST($1::BIGINT[]),
            UNNEST($2::TEXT[]),
            UNNEST($3::TEXT[]),
            UNNEST($4::BIGINT[]))
            "#,
            &ids,
            &names,
            &numbers,
            &buildings
        );

        query
            .execute(self)
            .await
            .wrap_err("Failed to insert multiple auditoriums")?;
        Ok(())
    }
}
