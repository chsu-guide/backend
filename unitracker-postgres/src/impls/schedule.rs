use crate::database::Database;
use eyre::{Result, WrapErr};
use sqlx::query::Query;
use sqlx::QueryBuilder;
use crate::models::auditoriums::DbAuditorium;
use crate::models::groups::DbGroup;
use crate::models::schedule::DbSchedule;
use crate::models::teachers::DbTeacher;

impl Database {
    pub async fn select_schedule(&self, id: i64) -> Result<Option<DbSchedule>> {
        let query = sqlx::query_as!(DbSchedule,
        r#"
        SELECT
            id, request_date, class_date, start_time, end_time, lesson_type, lesson_type_abbr, discipline_id
        FROM
            schedule
        WHERE
            id = $1
        "#,
            id
        );

        query.fetch_optional(self)
            .await
            .wrap_err("Failed to fetch schedule")
    }
    pub async fn insert_schedule(&self,
                                 db_schedule: DbSchedule,
                                 auditorium_id: i64,
                                 teacher_id: &[i64],
                                 group_id: &[i64]) -> Result<()> {
        let mut transaction = self.begin().await?;


        let schedule_auditorium_insert = sqlx::query!(r#"
        INSERT INTO schedule_auditorium
        (schedule_id, auditorium_id)
        VALUES ($1, $2)
        ON CONFLICT (schedule_id, auditorium_id) DO NOTHING
        "#,
        &db_schedule.id,
        auditorium_id)
            .execute(&mut *transaction)
            .await
            .wrap_err("schedule_auditorium insertion failed")?;
        for id in teacher_id {
            let schedule_teacher_insert = sqlx::query!(r#"
        INSERT INTO schedule_teacher
        (schedule_id, teacher_id)
        VALUES ($1, $2)
        ON CONFLICT (schedule_id, teacher_id) DO NOTHING
        "#,
        &db_schedule.id,
        id)
                .execute(&mut *transaction)
                .await
                .wrap_err("schedule_teacher insertion failed")?;
        }
        for id in group_id {
            let schedule_group_insert = sqlx::query!(r#"
        INSERT INTO schedule_group
        (schedule_id, group_id)
        VALUES ($1, $2)
        ON CONFLICT (schedule_id, group_id) DO NOTHING
        "#,
        &db_schedule.id,
        id)
                .execute(&mut *transaction)
                .await
                .wrap_err("schedule_group insertion failed")?;
        }

        let schedule_insert = sqlx::query!(r#"
        INSERT INTO schedule
        (id, request_date, class_date, start_time, end_time, lesson_type, lesson_type_abbr, discipline_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ON CONFLICT (id) DO NOTHING
        "#,
        db_schedule.id,
        db_schedule.request_date,
        db_schedule.class_date.into(),
        db_schedule.start_time,
        db_schedule.end_time,
        &db_schedule.lesson_type,
        db_schedule.lesson_type_abbr,
        db_schedule.discipline_id)
            .execute(&mut *transaction)
            .await
            .wrap_err("schedule insertion failed")?;

        transaction.commit().await.wrap_err("schedule transaction failed")?;
        Ok(())
    }
}