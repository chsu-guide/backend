use serde_derive::Deserialize;
#[derive(Debug, Default, Deserialize)]
pub struct Student {
    // Code of the group, used in request query
    pub code: String,
    // Human readable group name
    pub group: String,
}