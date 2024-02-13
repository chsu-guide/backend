use serde_derive::Deserialize;
pub type ClassList = Vec<Class>;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    pub build: Build,
    /// Auditory within the building
    pub auditory: Auditory,
    /// Lecturers reading the lecture
    pub lecturers: Vec<Lecturer>,
    /// Abbreviation of the lesson type (п, л, лб, экз)
    pub abbrlessontype: String,
    /// Full lesson type
    pub lessontype: String,
    /// Week index starting from September 1st
    pub week: i64,
    /// Day index starting from Monday
    pub weekday: i64,
    pub week_type: String,
    pub online_event: Option<String>,
    pub online: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Discipline {
    pub id: i64,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Group {
    pub id: i64,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Build {
    pub id: i64,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Auditory {
    pub id: i64,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Lecturer {
    pub id: i64,
    pub last_name: String,
    pub first_name: String,
    pub middle_name: String,
    pub short_name: String,
    pub fio: String,
}

