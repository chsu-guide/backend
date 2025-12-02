use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::models::{auditorium::Auditorium, group::GroupShort};

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Class {
    pub id: i64,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub lesson_type: Box<str>,
    pub lesson_type_abbreviated: Option<String>,
    pub discipline_name: Box<str>,
    pub auditorium_name: Vec<AuditoriumShort>,
    #[sqlx(flatten)]
    pub teacher_name: Vec<TeacherShort>,
    #[sqlx(flatten)]
    pub group_list: Vec<GroupShort>,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct ClassPartial {
    pub id: i64,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub lesson_type: Box<str>,
    pub lesson_type_abbreviated: Option<String>,
    pub discipline_name: Box<str>,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct AuditoriumShort {
    pub name: Box<str>,
    pub number: Box<str>,
    pub building_id: Option<i64>,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct TeacherShort {
    pub last_name: Box<str>,
    pub first_name: Box<str>,
    #[sqlx(default)]
    pub middle_name: Box<str>,
}
