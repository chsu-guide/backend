use eyre::{Context, Result};
use itertools::Itertools;

use crate::{database::Database, models::group::Group};

impl Database {
    #[tracing::instrument]
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
    pub async fn select_groups_all(&self) -> Result<Vec<Group>> {
        let query = sqlx::query_as!(
            Group,
            r#"
            SELECT id, name, course, faculty_id, chair_id
            FROM student_group
            "#
        );
        query
            .fetch_all(self)
            .await
            .wrap_err("Failed to fetch group list")
    }
    #[tracing::instrument]
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
    #[tracing::instrument]
    pub async fn insert_group_many(&self, group_list: &[Group]) -> Result<()> {
        for group in group_list {
            self.insert_group(group).await?
        }
        Ok(())
    }

    #[tracing::instrument]
    pub async fn initial_insert_groups_many(
        &self,
        group_list: &[unitracker_chsu::model::groups::Group],
    ) -> Result<()> {
        let (chair_id, chair_title): (Vec<_>, Vec<_>) = group_list
            .iter()
            .map(|g| (g.chair.id, g.chair.title.clone()))
            .dedup_by(|lhs, rhs| lhs.0 == rhs.0)
            .unzip();
        let _ = sqlx::query!(
            r#"
            INSERT INTO chair
            (id, name)
            SELECT * FROM UNNEST($1::bigint[], $2::text[])
            ON CONFLICT (id) DO NOTHING
            "#,
            &chair_id,
            &chair_title
        )
        .execute(self)
        .await?;

        let (faculty_id, faculty_name): (Vec<_>, Vec<_>) = group_list
            .iter()
            .map(|g| (g.faculty.id, g.faculty.title.clone()))
            .dedup_by(|lhs, rhs| lhs.0 == rhs.0)
            .unzip();
        let _ = sqlx::query!(
            r#"
            INSERT INTO faculty
            (id, name)
            SELECT * FROM UNNEST($1::bigint[], $2::text[])
            ON CONFLICT (id) DO NOTHING
            "#,
            &faculty_id,
            &faculty_name
        )
        .execute(self)
        .await?;

        let (group_id, group_title, group_course, group_faculty_id, group_chair_id): (
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
        ) = group_list
            .iter()
            .map(|g| {
                (
                    g.id,
                    g.title.clone(),
                    g.course as i16,
                    g.faculty.id,
                    g.chair.id,
                )
            })
            .multiunzip();
        let _ = sqlx::query!(
            r#"
            INSERT INTO student_group (id, name, course, chair_id, faculty_id)
            SELECT * FROM UNNEST($1::bigint[], $2::text[], $3::smallint[], $4::bigint[], $5::bigint[])
            ON CONFLICT (id) DO NOTHING
            "#,
            &group_id, &group_title, &group_course, &group_chair_id, &group_faculty_id).execute(self).await?;

        Ok(())
    }
}
