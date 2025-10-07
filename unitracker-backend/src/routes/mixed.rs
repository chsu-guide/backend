use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::util::types::IdOrName;

#[derive(Deserialize)]
struct MixedQuery {
    pub auditorium: IdOrName,
    pub teacher: IdOrName,
    pub group: IdOrName,
    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
}

fn get_mixed_schedule() {
    todo!()
}
