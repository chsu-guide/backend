use eyre::{Context, Result};

use crate::{database::Database, models::class::Class};

impl Database {
    pub async fn select_class(&self, id: i64) -> Result<Option<Class>> {
        let query = sqlx::query_as!(
            Class,
            r#"
            SELECT id, request_date AS created_at, start_time, end_time, lesson_type, lesson_type_abbr AS lesson_type_abbreviated, discipline_id
            FROM schedule
            WHERE id = $1
            "#,
            id
        );
        query
            .fetch_optional(self)
            .await
            .wrap_err("Failed to fetch class")
    }
    pub async fn insert_class(&self, class: &Class) -> Result<()> {
        let query = sqlx::query!(
            r#"
            INSERT INTO schedule
            (id, request_date, start_time, end_time, lesson_type, lesson_type_abbr, discipline_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO
            NOTHING
            "#,
            class.id,
            class.created_at,
            class.start_time,
            class.end_time,
            &class.lesson_type,
            class.lesson_type_abbreviated,
            class.discipline_id
        );

        query
            .execute(self)
            .await
            .wrap_err("Failed to insert a class")?;
        Ok(())
    }
    pub async fn insert_class_many(&self, class_list: &[Class]) -> Result<()> {
        for class in class_list.iter() {
            self.insert_class(class).await?
        }
        Ok(())
    }
}
