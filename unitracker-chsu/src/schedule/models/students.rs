use serde_derive::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Students {
    // Code of the group, used in request query
    pub(crate) code: String,
    // Human readable group name
    pub(crate) group: String,
}
