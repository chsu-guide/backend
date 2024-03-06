use serde_derive::Deserialize;
use crate::model::buildings::Building;
use crate::model::teachers::TeacherList;
use crate::model::disciplines::Discipline;

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Class {
    pub id: i64,
    /// Date formatted as dd.mm.YYYY
    pub date_event: String,
    /// Start time formatted as hh:mm
    pub start_time: String,
    /// End time formatted as hh:mm
    pub end_time: String,
    pub discipline: Discipline,
    /// Groups attending the class
    pub groups: Vec<Group>,
    /// Building the class is located in
    pub build: Option<Building>,
    /// Auditory within the building
    pub auditory: Option<Auditory>,
    /// Lecturers reading the lecture
    pub lecturers: Option<TeacherList>,
    /// Abbreviation of the lesson type (п, л, лб, экз)
    pub abbrlessontype: Option<String>,
    /// Full lesson type
    pub lessontype: Option<String>,
    /// Week index starting from September 1st
    pub week: i8,
    /// Day index starting from Monday
    pub weekday: i8,
    pub week_type: String,
    pub online_event: Option<String>,
    pub online: i8,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Group {
    pub id: i64,
    pub title: String,
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Auditory {
    pub id: i64,
    pub title: Option<String>,
}