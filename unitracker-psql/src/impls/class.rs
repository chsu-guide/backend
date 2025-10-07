use chrono::NaiveDateTime;
use eyre::{Context, Result, bail};
use itertools::Itertools;
use sqlx::query;
use sqlx::{
    Postgres,
    postgres::{PgArguments, PgRow},
    query::Map,
};
use unitracker_chsu::model::schedule::Schedule;
use unitracker_types::IdOrName;

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
    pub async fn select_class_by_group_with_timestamps(
        &self,
        id: i64,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Result<Vec<Class>> {
        let query = sqlx::query_as!(
            Class,
            r#"
            SELECT s.id, request_date AS created_at, start_time, end_time, lesson_type, lesson_type_abbr AS lesson_type_abbreviated, discipline_id
            FROM schedule s
            JOIN schedule_group sg ON s.id = sg.schedule_id
            JOIN student_group g ON sg.group_id = g.id
            WHERE g.id = $1 AND start_time > $2 AND end_time < $3
            "#,
            id,
            start,
            end
        );
        query
            .fetch_all(self)
            .await
            .wrap_err("Failed to fetch classes")
    }
    pub async fn select_class_by_name_with_timestamps(
        &self,
        name: String,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Result<Vec<Class>> {
        let query = sqlx::query_as!(
            Class,
            r#"
            SELECT schedule.id, request_date AS created_at, start_time, end_time, lesson_type, lesson_type_abbr AS lesson_type_abbreviated, discipline_id
            FROM schedule
            INNER JOIN discipline ON schedule.discipline_id = discipline.id
            WHERE discipline.name = $1 AND start_time > $2 AND end_time < $3
            "#,
            name,
            start,
            end
        );
        query
            .fetch_all(self)
            .await
            .wrap_err("Failed to fetch classes")
    }
    pub async fn select_class_by_name_and_group(
        &self,
        group_name: IdOrName,
        discipline_name: IdOrName,
    ) -> Result<Vec<Class>> {
        match (group_name, discipline_name) {
            (IdOrName::Id(group), IdOrName::Id(discipline)) => {
                self.class_select_query_ids(group, discipline).await
            }
            (IdOrName::Id(group), IdOrName::Name(discipline)) => {
                self.class_select_query_group_id_discipline_name(group, &discipline)
                    .await
            }
            (IdOrName::Name(group), IdOrName::Id(discipline)) => {
                self.class_select_query_group_name_discipline_id(&group, discipline)
                    .await
            }
            (IdOrName::Name(group), IdOrName::Name(discipline)) => {
                self.class_select_query_names(&group, &discipline).await
            }
        }
    }
    async fn class_select_query_ids<'a>(&self, group: i64, discipline: i64) -> Result<Vec<Class>> {
        let query = sqlx::query_as!(
            Class,
            r#"
            SELECT schedule.id, request_date AS created_at, start_time, end_time, lesson_type, lesson_type_abbr AS lesson_type_abbreviated, discipline_id
            FROM schedule
            INNER JOIN discipline ON schedule.discipline_id = discipline.id
            JOIN schedule_group sg ON schedule.id = sg.schedule_id
            JOIN student_group g ON sg.group_id = g.id
            WHERE g.id = $1 AND discipline.id = $2
            "#,
            group,
            discipline,
        );
        query
            .fetch_all(self)
            .await
            .wrap_err("Failed to fetch classes")
    }
    async fn class_select_query_group_name_discipline_id(
        &self,
        group: &str,
        discipline: i64,
    ) -> Result<Vec<Class>> {
        let query = sqlx::query_as!(
            Class,
            r#"
            SELECT schedule.id, request_date AS created_at, start_time, end_time, lesson_type, lesson_type_abbr AS lesson_type_abbreviated, discipline_id
            FROM schedule
            INNER JOIN discipline ON schedule.discipline_id = discipline.id
            JOIN schedule_group sg ON schedule.id = sg.schedule_id
            JOIN student_group g ON sg.group_id = g.id
            WHERE g.name = $1 AND discipline.id = $2
            "#,
            group,
            discipline,
        );
        query
            .fetch_all(self)
            .await
            .wrap_err("Failed to fetch classes")
    }
    async fn class_select_query_group_id_discipline_name(
        &self,
        group: i64,
        discipline: &str,
    ) -> Result<Vec<Class>> {
        let query = sqlx::query_as!(
            Class,
            r#"
            SELECT schedule.id, request_date AS created_at, start_time, end_time, lesson_type, lesson_type_abbr AS lesson_type_abbreviated, discipline_id
            FROM schedule
            INNER JOIN discipline ON schedule.discipline_id = discipline.id
            JOIN schedule_group sg ON schedule.id = sg.schedule_id
            JOIN student_group g ON sg.group_id = g.id
            WHERE g.id = $1 AND discipline.name = $2
            "#,
            group,
            discipline,
        );
        query
            .fetch_all(self)
            .await
            .wrap_err("Failed to fetch classes")
    }
    async fn class_select_query_names<'a>(
        &self,
        group: &'a str,
        discipline: &'a str,
    ) -> Result<Vec<Class>> {
        let query = sqlx::query_as!(
            Class,
            r#"
            SELECT schedule.id, request_date AS created_at, start_time, end_time, lesson_type, lesson_type_abbr AS lesson_type_abbreviated, discipline_id
            FROM schedule
            INNER JOIN discipline ON schedule.discipline_id = discipline.id
            JOIN schedule_group sg ON schedule.id = sg.schedule_id
            JOIN student_group g ON sg.group_id = g.id
            WHERE g.name = $1 AND discipline.name = $2
            "#,
            group,
            discipline,
        );
        query
            .fetch_all(self)
            .await
            .wrap_err("Failed to fetch classes")
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

        let res = query.execute(self).await;
        match res {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to insert class {class:#?} - {e}");
                bail!(e)
            }
        }

        Ok(())
    }
    pub async fn insert_class_many(&self, class_list: &[Class]) -> Result<()> {
        let (
            class_ids,
            class_creation_dates,
            class_start_times,
            class_end_times,
            lesson_types,
            lesson_abbreviations,
            discipline_ids,
        ): (Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>) = class_list
            .iter()
            .map(|c| {
                (
                    c.id,
                    c.created_at,
                    c.start_time,
                    c.end_time,
                    c.lesson_type.clone().to_string(),
                    c.lesson_type_abbreviated.clone().unwrap_or_default(),
                    c.discipline_id,
                )
            })
            .collect();
        // for class in class_list {
        //     self.insert_class(class).await?;
        // }
        let query = sqlx::query!(
            r#"
            INSERT INTO schedule
            (id, request_date, start_time, end_time, lesson_type, lesson_type_abbr, discipline_id)
            SELECT * FROM UNNEST($1::bigint[], $2::timestamp[], $3::timestamp[], $4::timestamp[], $5::text[], $6::text[], $7::bigint[])
            ON CONFLICT (id) DO
            NOTHING
            "#,
            &class_ids,
            &class_creation_dates,
            &class_start_times,
            &class_end_times,
            &lesson_types,
            &lesson_abbreviations,
            &discipline_ids as &[Option<i64>],
        ).execute(self).await?;

        Ok(())
    }

    pub async fn populate_classes(&self, schedule: &[Schedule]) -> Result<()> {
        // Populate schedule_auditorium
        let (schedules, groups, lecturers): (Vec<_>, Vec<_>, Vec<_>) = schedule
            .iter()
            .filter(|s| s.auditory.is_some())
            .filter(|s| s.auditory.as_ref().unwrap().id != 0)
            .filter(|s| s.lecturers.is_some())
            .map(|s| (s.id, s.groups.clone(), s.lecturers.as_ref().unwrap()))
            .multiunzip();
        let (schedules, teachers): (Vec<i64>, Vec<i64>) = schedule
            .iter()
            .filter(|s| s.lecturers.is_some())
            .map(|s| (s.id, s.lecturers.as_ref().unwrap()))
            .flat_map(|s| s.1.iter().map(move |val| (s.0, val.id)))
            .unzip();
        let query = sqlx::query!(
            r#"
            INSERT INTO schedule_teacher
            (schedule_id, teacher_id)
            SELECT * FROM UNNEST($1::bigint[], $2::bigint[])
            ON CONFLICT DO NOTHING
            "#,
            &schedules,
            &teachers
        )
        .execute(self)
        .await?;
        println!("Inserted {query:?} pairs");
        let (schedules, groups): (Vec<i64>, Vec<i64>) = schedule
            .iter()
            .flat_map(|s| s.groups.iter().map(move |val| (s.id, val.id)))
            .unzip();

        let query = sqlx::query!(
            r#"
            INSERT INTO schedule_group
            (schedule_id, group_id)
            SELECT * FROM UNNEST($1::bigint[], $2::bigint[])
            ON CONFLICT DO NOTHING
            "#,
            &schedules,
            &groups
        )
        .execute(self)
        .await?;
        println!("Inserted {query:?} pairs");
        let filtered: Vec<_> = schedule
            .iter()
            .filter(|s| s.auditory.is_some())
            .filter(|s| s.discipline.is_some())
            .filter(|s| s.lecturers.is_some())
            .collect();
        sqlx::query!("BEGIN").execute(self).await;
        for s in filtered {
            let s_a = sqlx::query!(
                "INSERT INTO schedule_auditorium (schedule_id, auditorium_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
                s.id,
                s.auditory.as_ref().unwrap().id
            )
            .execute(self)
            .await;
            match s_a {
                Ok(_) => {}
                Err(e) => {
                    eprintln!(
                        "Failed to insert auditorium {}",
                        s.auditory.as_ref().unwrap().id
                    );
                }
            }
        }
        sqlx::query!("COMMIT").execute(self).await;
        // let query = sqlx::query!(
        //     r#"
        //     INSERT INTO schedule_auditorium
        //     (schedule_id, auditorium_id)
        //     SELECT * FROM UNNEST($1::bigint[], $2::bigint[])
        //     ON CONFLICT DO NOTHING
        //     "#,
        //     &schedules,
        //     &auditoriums
        // )
        // .execute(self)
        // .await?;
        // println!("Inserted {query:?} pairs");
        // // Populate schedule_teacher
        Ok(())
    }
}
