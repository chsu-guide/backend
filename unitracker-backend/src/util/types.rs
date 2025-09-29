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

#[cfg(test)]
mod tests {
    use axum::{extract::Query, http::Uri};

    use crate::util::types::IdOrName;

    #[derive(PartialEq, Eq, serde::Deserialize, Debug)]
    struct TestQuery {
        foo: IdOrName,
        bar: u64,
    }
    #[test]
    fn test_query_de() {
        let uri: Uri = "http://example.com/path?foo=1&bar=222".parse().unwrap();
        let result: Query<TestQuery> = Query::try_from_uri(&uri).unwrap();
        assert_eq!(
            result.0,
            TestQuery {
                foo: IdOrName::Id(1),
                bar: 222,
            }
        );

        let uri: Uri = "http://example.com/path?foo=test&bar=111".parse().unwrap();
        let result: Query<TestQuery> = Query::try_from_uri(&uri).unwrap();
        assert_eq!(
            result.0,
            TestQuery {
                foo: IdOrName::Name(String::from("test")),
                bar: 111
            }
        )
    }
}
