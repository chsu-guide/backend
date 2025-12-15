use std::collections::HashMap;

use chrono::NaiveDateTime;
use eyre::{Context, OptionExt, Result, bail, eyre};
use itertools::Itertools;
use sqlx::{QueryBuilder, query::QueryAs};
use tracing::{Level, info, warn};
use unitracker_chsu::model::schedule::Schedule;
use unitracker_types::IdOrName;

use crate::{
    database::Database,
    models::{
        auditorium::Auditorium,
        class::Class,
        discipline,
        dtos::class::{AuditoriumShort, Class as DtoClass, ClassPartial, TeacherShort},
        group::{Group, GroupShort},
        teacher::Teacher,
    },
};

const BASE_STUDENT_SELECT: &'static str = r#"
SELECT s.id, start_time, end_time, lesson_type, lesson_type_abbr AS lesson_type_abbreviated,
    d.name AS discipline_name
FROM schedule s
INNER JOIN discipline d ON s.discipline_id = d.id
INNER JOIN schedule_group sg ON s.id = sg.schedule_id
INNER JOIN student_group g ON sg.group_id = g.id
"#;

const BASE_TEACHER_SELECT: &'static str = r#"
SELECT s.id, start_time, end_time, lesson_type, lesson_type_abbr AS lesson_type_abbreviated,
    d.name AS discipline_name
FROM schedule s
INNER JOIN discipline d ON s.discipline_id = d.id
INNER JOIN schedule_teacher st ON s.id = st.schedule_id
INNER JOIN teacher t ON st.teacher_id = t.id
"#;

const BASE_AUDITORIUM_SELECT: &'static str = r#"
SELECT s.id, start_time, end_time, lesson_type, lesson_type_abbr AS lesson_type_abbreviated,
    d.name AS discipline_name
FROM schedule s
INNER JOIN discipline d ON s.discipline_id = d.id
INNER JOIN schedule_auditorium sa ON s.id = sa.schedule_id
INNER JOIN auditorium a ON sa.auditorium_id = a.id
"#;
macro_rules! impl_student_query_variant {
    ($fn_name:ident, $query_type:ty, $query_subst:literal) => {
        #[tracing::instrument]
        async fn $fn_name(
            &self,
            id: $query_type,
            start: NaiveDateTime,
            end: NaiveDateTime,
        ) -> Result<Vec<DtoClass>> {
            let mut qb = QueryBuilder::new(BASE_STUDENT_SELECT);
            let qb = qb.push($query_subst);
            let query = qb.build_query_as().bind(id).bind(start).bind(end);
            let data: Vec<ClassPartial> = query
                .fetch_all(self)
                .await
                .wrap_err("Failed to fetch classes")
                .unwrap();

            if data.is_empty() {
                return Ok(vec![]);
            }
            let schedule_ids: Vec<i64> = data.iter().map(|s| s.id).collect();

            let mut group_map: HashMap<i64, Vec<GroupShort>> =
                self.select_groups_with_class_list(&schedule_ids).await;

            let mut teacher_map: HashMap<i64, Vec<TeacherShort>> =
                self.select_teachers_with_class_list(&schedule_ids).await;

            let mut auditoriums_map: HashMap<i64, Vec<Auditorium>> =
                self.select_auditoriums_with_class_list(&schedule_ids).await;
            let mut building_map: HashMap<i64, String> = self
                .select_building_all()
                .await?
                .into_iter()
                .map(|b| (b.id, b.name.to_string()))
                .collect();
            let mut list: Vec<DtoClass> = data
                .into_iter()
                .map(|c| DtoClass {
                    id: c.id,
                    start_time: c.start_time,
                    end_time: c.end_time,
                    lesson_type: c.lesson_type,
                    lesson_type_abbreviated: c.lesson_type_abbreviated,
                    discipline_name: c.discipline_name,
                    auditorium_name: auditoriums_map
                        .remove(&c.id)
                        .unwrap_or_default()
                        .into_iter()
                        .map(|a| AuditoriumShort {
                            name: a.name,
                            number: a.number,
                            building_id: a.building_id,
                            building_name: if let Some(bid) = a.building_id {
                                building_map.get(&bid).map(<_>::to_owned)
                            } else {
                                None
                            },
                        })
                        .collect(),
                    teacher_name: teacher_map.remove(&c.id).unwrap_or_default(),
                    group_list: group_map.remove(&c.id).unwrap_or_default(),
                })
                .collect();
            list.sort_unstable_by_key(|c| c.start_time);
            Ok(list)
        }
    };
}
macro_rules! impl_teacher_query_variant {
    ($fn_name:ident, $query_type:ty, $query_subst:literal) => {
        #[tracing::instrument]
        async fn $fn_name(
            &self,
            id: $query_type,
            start: NaiveDateTime,
            end: NaiveDateTime,
        ) -> Result<Vec<DtoClass>> {
            let mut qb = QueryBuilder::new(BASE_TEACHER_SELECT);
            let qb = qb.push($query_subst);
            let query = qb.build_query_as().bind(id).bind(start).bind(end);
            let data: Vec<ClassPartial> = query
                .fetch_all(self)
                .await
                .wrap_err("Failed to fetch classes")
                .unwrap();

            if data.is_empty() {
                return Ok(vec![]);
            }
            let schedule_ids: Vec<i64> = data.iter().map(|s| s.id).collect();

            let mut group_map: HashMap<i64, Vec<GroupShort>> =
                self.select_groups_with_class_list(&schedule_ids).await;

            let mut auditoriums_map: HashMap<i64, Vec<Auditorium>> =
                self.select_auditoriums_with_class_list(&schedule_ids).await;

            let mut teacher_map: HashMap<i64, Vec<TeacherShort>> =
                self.select_teachers_with_class_list(&schedule_ids).await;

            let mut building_map: HashMap<i64, String> = self
                .select_building_all()
                .await?
                .into_iter()
                .map(|b| (b.id, b.name.to_string()))
                .collect();

            let mut list: Vec<DtoClass> = data
                .into_iter()
                .map(|c| DtoClass {
                    id: c.id,
                    start_time: c.start_time,
                    end_time: c.end_time,
                    lesson_type: c.lesson_type,
                    lesson_type_abbreviated: c.lesson_type_abbreviated,
                    discipline_name: c.discipline_name,
                    auditorium_name: auditoriums_map
                        .remove(&c.id)
                        .unwrap_or_default()
                        .into_iter()
                        .map(|a| AuditoriumShort {
                            name: a.name,
                            number: a.number,
                            building_id: a.building_id,
                            building_name: if let Some(bid) = a.building_id {
                                building_map.get(&bid).map(<_>::to_owned)
                            } else {
                                None
                            },
                        })
                        .collect(),
                    group_list: group_map.remove(&c.id).unwrap_or_default(),
                    teacher_name: teacher_map.remove(&c.id).unwrap_or_default(),
                })
                .collect();
            list.sort_unstable_by_key(|c| c.start_time);
            Ok(list)
        }
    };
}

macro_rules! impl_auditorium_query_variant {
    ($fn_name:ident, $query_type:ty, $query_subst:literal) => {
        #[tracing::instrument]
        pub async fn $fn_name(
            &self,
            id: $query_type,
            start: NaiveDateTime,
            end: NaiveDateTime,
        ) -> Result<Vec<DtoClass>> {
            let mut qb = QueryBuilder::new(BASE_AUDITORIUM_SELECT);
            let qb = qb.push($query_subst);
            let query = qb.build_query_as().bind(id).bind(start).bind(end);
            let data: Vec<ClassPartial> = query
                .fetch_all(self)
                .await
                .wrap_err("Failed to fetch classes")
                .unwrap();

            if data.is_empty() {
                return Ok(vec![]);
            }
            let schedule_ids: Vec<i64> = data.iter().map(|s| s.id).collect();

            let mut group_map: HashMap<i64, Vec<GroupShort>> =
                self.select_groups_with_class_list(&schedule_ids).await;

            let mut teacher_map: HashMap<i64, Vec<TeacherShort>> =
                self.select_teachers_with_class_list(&schedule_ids).await;

            let mut auditoriums_map: HashMap<i64, Vec<Auditorium>> =
                self.select_auditoriums_with_class_list(&schedule_ids).await;
            let mut building_map: HashMap<i64, String> = self
                .select_building_all()
                .await?
                .into_iter()
                .map(|b| (b.id, b.name.to_string()))
                .collect();

            let mut list: Vec<DtoClass> = data
                .into_iter()
                .map(|c| DtoClass {
                    id: c.id,
                    start_time: c.start_time,
                    end_time: c.end_time,
                    lesson_type: c.lesson_type,
                    lesson_type_abbreviated: c.lesson_type_abbreviated,
                    discipline_name: c.discipline_name,
                    auditorium_name: auditoriums_map
                        .remove(&c.id)
                        .unwrap_or_default()
                        .into_iter()
                        .map(|a| AuditoriumShort {
                            name: a.name,
                            number: a.number,
                            building_id: a.building_id,
                            building_name: if let Some(bid) = a.building_id {
                                building_map.get(&bid).map(<_>::to_owned)
                            } else {
                                None
                            },
                        })
                        .collect(),
                    group_list: group_map.remove(&c.id).unwrap_or_default(),
                    teacher_name: teacher_map.remove(&c.id).unwrap_or_default(),
                })
                .collect();
            list.sort_unstable_by_key(|c| c.start_time);
            Ok(list)
        }
    };
}
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
    ) -> Result<Vec<DtoClass>> {
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

    pub async fn select_class_by_teacher_with_timestamps(
        &self,
        teacher: IdOrName,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Result<Vec<DtoClass>> {
        match teacher {
            IdOrName::Id(id) => {
                self.select_class_by_teacher_id_with_timestamps(id, start, end)
                    .await
            }
            IdOrName::Name(name) => {
                self.select_class_by_teacher_name_with_timestamps(name, start, end)
                    .await
            }
        }
    }
    pub async fn select_class_by_auditorium_with_timestamps(
        &self,
        auditorium: IdOrName,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Result<Vec<DtoClass>> {
        match auditorium {
            IdOrName::Id(id) => {
                self.select_class_by_auditorium_id_with_timestamps(id, start, end)
                    .await
            }
            IdOrName::Name(name) => {
                self.select_class_by_auditorium_name_with_timestamps(name, start, end)
                    .await
            }
        }
    }
    impl_student_query_variant!(
        select_class_by_group_id_with_timestamps,
        i64,
        "WHERE g.id = $1 AND start_time > $2 AND end_time < $3"
    );
    impl_student_query_variant!(
        select_class_by_group_name_with_timestamps,
        String,
        "WHERE g.name = $1 AND start_time > $2 AND end_time < $3"
    );
    impl_teacher_query_variant!(
        select_class_by_teacher_id_with_timestamps,
        i64,
        "WHERE t.id = $1 AND start_time > $2 AND end_time < $3"
    );
    impl_teacher_query_variant!(
        select_class_by_teacher_name_with_timestamps,
        String,
        "WHERE concat(t.last_name, ' ', t.first_name, ' ', t.middle_name) = $1 AND start_time > $2 AND end_time < $3"
    );
    impl_auditorium_query_variant!(
        select_class_by_auditorium_id_with_timestamps,
        i64,
        "WHERE a.id = $1 AND start_time > $2 AND end_time < $3"
    );
    impl_auditorium_query_variant!(
        select_class_by_auditorium_name_with_timestamps,
        String,
        "WHERE a.name = $1 AND start_time > $2 AND end_time < $3"
    );
    async fn select_auditoriums_with_class_list(
        &self,
        list: &[i64],
    ) -> HashMap<i64, Vec<Auditorium>> {
        let auditoriums: Vec<Auditorium> = sqlx::query_as(
            "SELECT sa.schedule_id AS id, a.name, a.number, a.building_id
                 FROM schedule_auditorium sa
                 INNER JOIN auditorium a ON sa.auditorium_id = a.id
                 WHERE sa.schedule_id = ANY($1)",
        )
        .bind(&list)
        .fetch_all(self)
        .await
        .unwrap();
        let mut auditoriums_map: HashMap<i64, Vec<Auditorium>> = HashMap::new();
        for auditorium in auditoriums {
            auditoriums_map
                .entry(auditorium.id)
                .or_insert_with(Vec::new)
                .push(auditorium);
        }
        auditoriums_map
    }
    async fn select_teachers_with_class_list(
        &self,
        list: &[i64],
    ) -> HashMap<i64, Vec<TeacherShort>> {
        let teachers: Vec<Teacher> = sqlx::query_as(
            r#"SELECT st.schedule_id AS id, t.last_name, t.first_name, t.middle_name
            FROM schedule_teacher st
            INNER JOIN teacher t ON st.teacher_id = t.id
            WHERE st.schedule_id = ANY($1)
            "#,
        )
        .bind(&list)
        .fetch_all(self)
        .await
        .wrap_err("Failed to fetch teachers for classes")
        .unwrap();

        let mut teacher_map: HashMap<i64, Vec<TeacherShort>> = HashMap::new();
        for teacher in teachers {
            teacher_map
                .entry(teacher.id)
                .or_insert_with(Vec::new)
                .push(TeacherShort {
                    last_name: teacher.last_name,
                    first_name: teacher.first_name,
                    middle_name: teacher.middle_name.unwrap_or_default().into_boxed_str(),
                });
        }
        teacher_map
    }

    async fn select_groups_with_class_list(&self, list: &[i64]) -> HashMap<i64, Vec<GroupShort>> {
        dbg!(&list);
        let groups: Vec<GroupShort> = sqlx::query_as(
            r#"
            SELECT sg.schedule_id AS id, name, course
            FROM schedule_group sg
            INNER JOIN student_group g ON sg.group_id = g.id
            WHERE sg.schedule_id = ANY($1)
            "#,
        )
        .bind(&list)
        .fetch_all(self)
        .await
        .wrap_err("Failed to fetch groups for classes")
        .unwrap();
        dbg!(&groups);

        let mut group_map: HashMap<i64, Vec<GroupShort>> = HashMap::new();
        for group in groups {
            group_map
                .entry(group.id)
                .or_insert_with(Vec::new)
                .push(GroupShort {
                    id: group.id,
                    name: group.name,
                    course: group.course,
                });
        }
        dbg!(&group_map);
        group_map
    }
    pub async fn select_class_by_teacher(
        &self,
        teacher: IdOrName,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Result<Vec<DtoClass>> {
        todo!()
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
                    name: discipline.into(),
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
        for s in schedule.iter().dedup_by(|lhs, rhs| lhs.id.eq(&rhs.id)) {
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
            })
            .dedup();
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
            .flat_map(|s| s.groups.iter().map(move |val| (s.id, val.id)))
            .dedup();

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
            .dedup_by(|lhs, rhs| lhs.id.eq(&rhs.id))
            .collect();

        let auditorium_pre_data: HashMap<_, _> = schedule
            .iter()
            .filter(|sc| sc.auditory.is_some())
            .map(|schedule_item| {
                (
                    schedule_item.id,
                    (
                        schedule_item.auditory.as_ref().unwrap().id,
                        schedule_item.auditory.as_ref().unwrap().title.clone(),
                        schedule_item.build.as_ref().unwrap().id,
                        schedule_item.build.as_ref().unwrap().title.clone(),
                    ),
                )
            })
            .collect();

        let auditoriums: HashMap<_, _> = self
            .select_auditorium_all()
            .await?
            .into_iter()
            .map(|a| ((a.number, a.building_id.unwrap_or_default()), a.id))
            .dedup()
            .collect();

        // for ((name, bid), id) in &auditoriums {
        // println!("name {name} in {bid} with id {id}");
        // }
        // panic!();

        let buildings: HashMap<_, _> = self
            .select_building_all()
            .await
            .unwrap()
            .iter()
            .map(|b| (b.name.clone(), b.id))
            .collect();

        let transaction = self.begin().await?;
        for s in filtered {
            let auditorium_data = auditorium_pre_data.get(&s.id);
            // dbg!(auditorium_data);
            if let None = auditorium_data {
                continue;
            }
            let (api_aud_id, api_aud_title, api_build_id, api_build_title) =
                auditorium_data.unwrap();
            let inner_building = buildings.get(api_build_title.clone().into_boxed_str().as_ref());
            // dbg!(inner_building);
            if let None = inner_building {
                continue;
            }
            let auditorium = auditoriums.get(&(
                api_aud_title.clone().unwrap().into_boxed_str(),
                *inner_building.unwrap(),
            ));
            if let None = auditorium {
                continue;
            }
            let auditorium = auditorium.unwrap();

            let s_a = sqlx::query!(
                "INSERT INTO schedule_auditorium (schedule_id, auditorium_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
                s.id,
                auditorium
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
            // self.insert_class(
            //     &Class::from(s.to_owned()),
            //     &s.discipline.as_ref().map(|d| d.title.clone()).unwrap(),
            // )
            // .await?;
        }
        transaction.commit().await?;

        Ok(())
    }
}
