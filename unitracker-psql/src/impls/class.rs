use std::collections::HashMap;

use chrono::NaiveDateTime;
use eyre::{Context, OptionExt, Result, bail};
use tracing::{Level, info, warn};
use unitracker_chsu::model::schedule::Schedule;
use unitracker_types::IdOrName;

use crate::{
    database::Database,
    models::{class::Class, discipline, teacher::Teacher},
};

impl Database {
    /// Select a Class by ID
    #[tracing::instrument]
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
        group: IdOrName,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Result<Vec<Class>> {
        match group {
            IdOrName::Id(id) => {
                self.select_class_by_group_id_with_timestamps(id, start, end)
                    .await
            }
            IdOrName::Name(name) => {
                self.select_class_by_group_name_with_timestamps(name, start, end)
                    .await
            }
        }
    }
    /// Select a list of classes by group ID within a certain range of dates
    #[tracing::instrument]
    async fn select_class_by_group_id_with_timestamps(
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
    /// Select a list of classes by group name within a certain date range
    #[tracing::instrument]
    async fn select_class_by_group_name_with_timestamps(
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
            JOIN schedule_group sg ON schedule.id = sg.schedule_id
            JOIN student_group g ON sg.group_id = g.id
            WHERE g.name = $1 AND start_time > $2 AND end_time < $3
            "#,
            name,
            start,
            end
        );
        let res = query.fetch_all(self).await;
        info!("result: {res:?}");

        res.wrap_err("Failed to fetch classes")
    }
    /// Select a class by group and discipline
    #[tracing::instrument(
        skip(self, group_name, discipline_name),
        err(Debug, level = Level::WARN))]
    pub async fn select_class_by_group_and_discipline(
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
    #[tracing::instrument]
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
    #[tracing::instrument]
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
        let ret = query
            .fetch_all(self)
            .await
            .wrap_err("Failed to fetch classes")?;

        if ret.is_empty() {
            bail!("Fetched zero classes");
        }
        Ok(ret)
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
    /// Insert a class
    pub async fn insert_class(&self, class: &Class, discipline: &str) -> Result<()> {
        let disc = self.select_discipline_by_name(discipline).await?;
        let d = match disc {
            Some(d) => d,
            None => {
                self.insert_discipline(&discipline::Discipline {
                    id: 0,
                    name: discipline.clone().into(),
                })
                .await?;
                self.select_discipline_by_name(discipline).await?.unwrap()
            }
        };
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
            d.id
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
    // Mass-insert classes
    // WARNING: Very heavy on RAM due to unnesting
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
        let _ = sqlx::query!(
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

    // Mass-insert classes and related entities
    // WARNING: Very heavy on RAM due to unnesting
    pub async fn populate_classes(&self, schedule: &[Schedule]) -> Result<()> {
        for s in schedule {
            self.insert_class(
                &Class::from(s.to_owned()),
                &s.discipline.as_ref().map(|d| d.title.clone()).unwrap(),
            )
            .await?;
        }
        let teachers: HashMap<(Box<str>, Box<str>), i64> = self
            .select_teacher_all()
            .await?
            .into_iter()
            .map(|t| ((t.last_name, t.first_name), t.id))
            .collect();

        let teacher_ref = &teachers;

        let schedule_teacher_pairs = schedule
            .iter()
            .filter(|s| s.lecturers.is_some())
            .map(|s| (s.id, s.lecturers.as_ref().unwrap()))
            .flat_map(|(id, teach)| {
                teach.into_iter().map(move |t| {
                    (
                        id,
                        teacher_ref.get(&(
                            t.last_name.clone().into_boxed_str(),
                            t.first_name.clone().into_boxed_str(),
                        )),
                    )
                })
            });
        // .flat_map(|s| {
        //     s.1.into_iter().map(move |val| async {
        //         (
        //             s.0.clone(),
        //             teacher_ref
        //                 .get(&(
        //                     val.last_name.clone().into_boxed_str(),
        //                     val.first_name.clone().into_boxed_str(),
        //                 ))
        //                 .map(Clone::clone)
        //                 .unwrap_or(
        //                     self.insert_teacher(&Teacher {
        //                         id: val.id,
        //                         last_name: val.last_name.clone().into_boxed_str(),
        //                         first_name: val.first_name.clone().into_boxed_str(),
        //                         middle_name: val.middle_name.clone(),
        //                     })
        //                     .await
        //                     .unwrap(),
        //                 ),
        //         )
        //     })
        // });

        let transaction = self.begin().await?;
        println!("Started importing schedule_teachers:");
        for (id, teacher) in schedule_teacher_pairs {
            if let Some(t) = teacher {
                let _ = sqlx::query!(
                    r#"
                INSERT INTO schedule_teacher
                (schedule_id, teacher_id)
                VALUES ($1, $2)
                ON CONFLICT DO NOTHING
                "#,
                    id,
                    t
                )
                .execute(self)
                .await
                .wrap_err("Failed to insert schedule-teacher pair")?;
            }
        }
        transaction.commit().await?;

        let schedule_group_pairs = schedule
            .iter()
            .flat_map(|s| s.groups.iter().map(move |val| (s.id, val.id)));

        println!("Started importing schedule_group:");
        let transaction = self.begin().await?;
        for (sched, group) in schedule_group_pairs {
            let _ = sqlx::query!(
                r#"
                INSERT INTO schedule_group
                (schedule_id, group_id)
                VALUES ($1, $2)
                ON CONFLICT DO NOTHING
                "#,
                sched,
                group
            )
            .execute(self)
            .await
            .wrap_err("Failed to insert schedule-group pair")?;
        }
        transaction.commit().await?;
        let filtered: Vec<_> = schedule
            .iter()
            .filter(|s| s.auditory.is_some())
            .filter(|s| s.discipline.is_some())
            .filter(|s| s.lecturers.is_some())
            .collect();
        println!("Started importing schedule_auditorium:");

        let auditoriums: HashMap<_, _> = self
            .select_auditorium_all()
            .await?
            .into_iter()
            .map(|a| (a.name, a.id))
            .collect();

        let transaction = self.begin().await?;
        for s in filtered {
            let aud = s
                .auditory
                .as_ref()
                .and_then(|a| auditoriums.get(&a.title.as_ref().unwrap().clone().into_boxed_str()));
            let s_a = sqlx::query!(
                "INSERT INTO schedule_auditorium (schedule_id, auditorium_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
                s.id,
                aud as Option<&i64>
            )
            .execute(self)
            .await;
            match s_a {
                Ok(_) => {}
                Err(e) => {
                    warn!(
                        "Failed to insert auditorium {} : {e}",
                        s.auditory.as_ref().unwrap().id
                    );
                }
            }
            self.insert_class(
                &Class::from(s.to_owned()),
                &s.discipline.as_ref().map(|d| d.title.clone()).unwrap(),
            )
            .await?;
        }
        transaction.commit().await?;

        Ok(())
    }
}
