use serde::{
    Deserialize,
    de::{self},
};

#[derive(Debug, PartialEq, Eq)]
pub enum IdOrName {
    Id(i64),
    Name(String),
}

/// NOTE: This way only works on URL-encode as it can't handle an actual raw u64 value
impl<'de> Deserialize<'de> for IdOrName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        match s.parse::<i64>() {
            Ok(val) => Ok(IdOrName::Id(val)),
            Err(_) => Ok(IdOrName::Name(s)),
        }
    }
}
