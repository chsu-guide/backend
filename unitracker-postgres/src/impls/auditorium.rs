use eyre::{Result, WrapErr};
use sqlx::{Executor, Postgres, QueryBuilder, Transaction};
use crate::database::Database;
use unitracker_chsu::model::auditoriums::{Auditorium as ApiAuditorium, AuditoriumList as ApiAuditoriumList};
use unitracker_chsu::model::buildings::{Building as ApiBuilding, BuildingList as ApiBuildingList};
use crate::models::auditorium::DbAuditorium;

impl Database {
    pub async fn insert_auditorium(&self, api_auditorium: ApiAuditorium) -> Result<()> {
        let mut transaction = self
            .begin()
            .await
            .wrap_err("insert_auditorium: transaction failed to start")?;

        self.insert_building(ApiBuilding {id: api_auditorium.building_id, title: api_auditorium.building_name}).await?;

        let query = sqlx::query!(r#"
        INSERT INTO auditorium
        (id, name, number, height, length, width, building_id)
        VALUES
        ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (id) DO
        UPDATE SET
        name = $2,
        number = $3,
        height = $4,
        length = $5,
        width = $6,
        building_id = $7
        "#,
        api_auditorium.id,
        api_auditorium.name,
        api_auditorium.number,
        api_auditorium.height,
        api_auditorium.length,
        api_auditorium.width,
        api_auditorium.building_id
        ).execute(&mut *transaction)
            .await
            .wrap_err("insert_auditorium: insertion failed")?;

        transaction.commit().await.wrap_err("insert_auditorium: transaction failed")?;
        Ok(())
    }

    pub async fn insert_auditorium_many(&self, api_auditorium_list: ApiAuditoriumList) -> Result<()> {
        let mut transaction = self
            .begin()
            .await
            .wrap_err("insert_auditorium: transaction failed to start")?;

        let ids: Vec<_> = api_auditorium_list.iter().clone().map(|x| x.id).collect();
        let names: Vec<_> = api_auditorium_list.iter().clone().map(|x| x.clone().name).collect();
        let numbers: Vec<_> = api_auditorium_list.iter().clone().map(|x| x.clone().number).collect();
        let heights: Vec<_> = api_auditorium_list.iter().clone().map(|x| x.height).collect();
        let lengths: Vec<_> = api_auditorium_list.iter().clone().map(|x| x.length).collect();
        let widths: Vec<_> = api_auditorium_list.iter().clone().map(|x| x.width).collect();
        let buildings: ApiBuildingList = api_auditorium_list.iter().clone().map(|x| ApiBuilding { id: x.building_id, title: x.building_name.to_owned()}).collect();
        let building_ids: Vec<_> = api_auditorium_list.iter().map(|x| x.building_id).collect();

        self.insert_building_many(buildings).await?;

        let mut auditorium_query = sqlx::query!(r#"
        INSERT INTO auditorium
        (id, name, number, height, length, width, building_id)
        SELECT * FROM UNNEST($1::int8[], $2::text[], $3::text[], $4::float4[], $5::float4[], $6::float4[], $7::int8[])
        ON CONFLICT (id) DO UPDATE SET
            name = excluded.name,
            number = excluded.number,
            height = excluded.height,
            length = excluded.length,
            width = excluded.width,
            building_id = excluded.building_id
        "#,
        &ids, &names, &numbers, &heights, &lengths, &widths, &building_ids).execute(&mut *transaction).await.wrap_err("insert_auditorium_many: insertion failed")?;

        transaction.commit().await.wrap_err("insert_auditorium: transaction failed")?;
        Ok(())
    }

    pub async fn select_auditorium(&self, auditorium_id: i64) -> Result<Option<DbAuditorium>> {
        sqlx::query_as!(DbAuditorium, r#"
        SELECT
            id, name, number, height, width, length, building_id
        FROM
            auditorium
        WHERE
            id = $1
        "#, auditorium_id).fetch_optional(self).await.wrap_err("select_auditorium: query failed")
    }
}