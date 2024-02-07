use serde_derive::Deserialize;
#[derive(Debug, Default, Deserialize)]
pub struct Tutor {
    // Code of the tutor, used in request query
    pub code: String,
    // Human readable tutor name
    pub group: String,
}
