pub struct DbGroup {
    pub id: u64,
    pub title: Box<str>,
    pub course: i8,
    pub faculty_id: Option<i64>,
    pub chair_id: Option<i64>,
}