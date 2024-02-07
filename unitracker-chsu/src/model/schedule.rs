use serde_derive::Deserialize;
#[derive(Default, Debug, Deserialize)]
pub struct Week {
    /// Index of the week starting at September 1st
    week: i16,
    /// Vector of [`Date`] structs
    dates: Vec<Date>,
}
#[derive(Default, Debug, Deserialize)]
pub struct Date {
    /// Date string formatted as dd.MM.yyyy
    date: String,
    /// Index of the day in the week
    day: i8,
    /// Vector of [`Class`] structs
    classes: Vec<Class>,
}
#[derive(Default, Debug, Deserialize)]
pub struct Class {
    /// Start and end time of the class, formatted as 'hh:mm - hh:mm'
    time: String,
    /// Type of the class
    class_type: String,
    /// Name of the class
    name: String,
    /// Location of the class
    location: String,
    /// Classroom number as a string (for whatever reason)
    classroom: String,
    /// Groups that should visit the class
    groups: Vec<String>,
    /// Teachers
    teachers: Vec<String>,
}
