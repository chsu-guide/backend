use eyre::{Context, Result};

use crate::{database::Database, models::group::Group};

impl Database {
    pub async fn select_group(&self, id: i64) -> Result<Option<Group>> {
        let query = sqlx::query_as!(
            Group,
            r#"
            SELECT id, name, course, faculty_id, chair_id
            FROM student_group
            WHERE id = $1
            "#,
            id
        );
        query
            .fetch_optional(self)
            .await
            .wrap_err("Failed to fetch discipline")
    }
    pub async fn insert_group(&self, group: &Group) -> Result<()> {
        let query = sqlx::query!(
            r#"
            INSERT INTO student_group
            (id, name, course, faculty_id, chair_id)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO
            NOTHING
            "#,
            group.id,
            &group.name,
            group.course,
            group.faculty_id,
            group.chair_id
        );

        query
            .execute(self)
            .await
            .wrap_err("Failed to insert a class")?;
        Ok(())
    }
    pub async fn insert_group_many(&self, group_list: &[Group]) -> Result<()> {
        for group in group_list {
            self.insert_group(group).await?
        }
        Ok(())
    }
}
