use crate::{database::Database, models::class::Class};

impl Database {
    pub fn get_mixed_schedule(&self, teacher: String, group: String) -> Vec<Class> {
        sqlx::query_as!(
            Class,
            r#"
            SELECT s.id, request_date AS created_at, start_time, end_time, lesson_type, lesson_type_abbr AS lesson_type_abbreviated, discipline_id
            FROM schedule s
            INNER JOIN schedule_teacher s_t ON s.id = s_t.schedule_id
            INNER JOIN teacher t ON t.id = s_t.teacher_id AND t.last_name = $1
            INNER JOIN schedule_group s_g ON s.id = s_g.schedule_id
            INNER JOIN student_group g ON g.id = s_g.schedule_id AND g.name = $2
            "#,
            teacher,
            group
        );
        todo!();
    }
}
