use crate::database::Database;
use eyre::{Result, WrapErr};
use sqlx::{Postgres, QueryBuilder, Transaction};
use unitracker_chsu::model::buildings::{Building as ApiBuilding, BuildingList as ApiBuildingList};

impl Database {
    pub async fn insert_building(&self, building: ApiBuilding) -> Result<()> {
        let mut transaction = self
            .begin()
            .await
            .wrap_err("insert_building: transaction failed to start")?;

        let query = sqlx::query!(r#"
        INSERT INTO building
        (id, title)
        VALUES
        ($1, $2)
        ON CONFLICT (id) DO
        UPDATE SET
        title = $2
        "#,
        building.id,
        building.title,
        ).execute(&mut *transaction)
            .await
            .wrap_err("insert_building: insertion failed")?;

        transaction.commit().await.wrap_err("insert_building: transaction failed")?;
        Ok(())
    }

    pub async fn insert_building_many(&self, building_list: ApiBuildingList) -> Result<()> {
        let mut transaction = self
            .begin()
            .await
            .wrap_err("insert_building_many: transaction failed to start")?;

        let ids: Vec<_> = building_list.iter().clone().map(|x| x.id).collect();
        let titles: Vec<_> = building_list.iter().clone().map(|x| x.clone().title).collect();

        let mut building_query = sqlx::query!(r#"
        INSERT INTO building
        (id, title)
        SELECT * FROM UNNEST($1::int8[], $2::text[])
        ON CONFLICT (id) DO NOTHING
        "#,
        &ids, &titles).execute(&mut *transaction).await.wrap_err("insert_building_many: insertion failed")?;

        transaction.commit().await.wrap_err("insert_building_many: transaction failed")?;
        Ok(())
    }

    pub async fn select_building(&self) {
        todo!()
    }
}