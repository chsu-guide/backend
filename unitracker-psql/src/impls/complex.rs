use chrono::NaiveDateTime;
use eyre::Context;
use sqlx::QueryBuilder;
use unitracker_types::IdOrName;

use crate::{database::Database, models::class::Class};

impl Database {
    pub async fn get_mixed_schedule(
        &self,
        teacher: Option<IdOrName>,
        group: Option<IdOrName>,
        discipline: Option<IdOrName>,
        start_date: Option<NaiveDateTime>,
        end_date: Option<NaiveDateTime>,
    ) -> eyre::Result<Vec<Class>> {
        let mut qb: QueryBuilder<'_, sqlx::Postgres> = QueryBuilder::new(
            r#"
            SELECT s.id, request_date AS created_at, start_time, end_time, lesson_type, lesson_type_abbr AS lesson_type_abbreviated, discipline_id
            FROM schedule s
            "#,
        );

        match teacher {
            Some(t) => {
                qb.push(
                    r#"
                    INNER JOIN schedule_teacher s.t. ON s.id = s_t.schedule_id
                    INNER JOIN teacher t ON t.id = s_t.teacher_id
                    "#,
                );

                match t {
                    IdOrName::Id(id) => qb.push(r#"WHERE t.id ="#).push_bind(id),
                    IdOrName::Name(name) => qb
                        .push(r#"WHERE t.last_name || t.first_name || t.middle_name = "#)
                        .push_bind(name.split_whitespace().collect::<String>()),
                };
            }
            None => (),
        }
        match group {
            Some(g) => {
                qb.push(
                    r#"
                    INNER JOIN schedule_group s_g ON s.id = s_g.group_id
                    INNER JOIN group g ON g.id = s_g.group_id
                    "#,
                );
                match g {
                    IdOrName::Id(id) => qb.push("WHERE g.id = ").push_bind(id),
                    IdOrName::Name(name) => qb.push("WHERE g.name =").push_bind(name),
                };
            }
            None => {}
        }

        match discipline {
            Some(d) => {
                qb.push(
                    r#"
                    JOIN discipline d on s.discipline_id = d.id
                    "#,
                );
                match d {
                    IdOrName::Id(id) => qb.push(r#"WHERE d.id = "#).push_bind(id),
                    IdOrName::Name(name) => qb.push(r#"WHERE d.name = "#).push_bind(name),
                };
            }
            None => (),
        }

        if let Some(date) = start_date {
            qb.push("WHERE start_time >").push_bind(date);
        }

        if let Some(date) = end_date {
            qb.push("WHERE end_time <").push_bind(date);
        }

        let query: Vec<Class> = qb
            .build_query_as::<Class>()
            .fetch_all(self)
            .await
            .wrap_err("Failed to select a schedule")?;
        Ok(query)
    }
}
