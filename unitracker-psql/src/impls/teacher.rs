use eyre::{Context, Result};

use crate::{database::Database, models::teacher::Teacher};

impl Database {
    #[tracing::instrument]
    pub async fn select_teacher(&self, id: i64) -> Result<Option<Teacher>> {
        let query = sqlx::query_as!(
            Teacher,
            r#"
            SELECT id, last_name, first_name, middle_name
            FROM teacher
            WHERE id = $1
            "#,
            id
        );
        query
            .fetch_optional(self)
            .await
            .wrap_err("Failed to fetch discipline")
    }
    pub async fn select_teacher_all(&self) -> Result<Vec<Teacher>> {
        let query = sqlx::query_as!(
            Teacher,
            r#"
            SELECT id, last_name, first_name, middle_name
            FROM teacher
            "#,
        );
        query
            .fetch_all(self)
            .await
            .wrap_err("Failed to fetch teachers")
    }
    #[tracing::instrument]
    pub async fn insert_teacher(&self, teacher: &Teacher) -> Result<i64> {
        let query = sqlx::query!(
            r#"
            INSERT INTO teacher
            (id, last_name, first_name, middle_name)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (id) DO
            NOTHING
            RETURNING id
            "#,
            teacher.id,
            &teacher.last_name,
            &teacher.first_name,
            teacher.middle_name,
        );

        let id = query
            .fetch_one(self)
            .await
            .wrap_err("Failed to insert a class")?;
        Ok(id.id)
    }
    #[tracing::instrument]
    pub async fn insert_teacher_many(&self, teacher_list: &[Teacher]) -> Result<()> {
        let tran = self.begin().await?;

        for teacher in teacher_list {
            let _ = self.insert_teacher(teacher).await?;
        }
        tran.commit().await?;

        Ok(())
    }
}
