use axum::extract::Query;
use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::util::types::IdOrName;

#[derive(Deserialize)]
struct AuditoriumQuery {
    auditorium: IdOrName,
    start: NaiveDateTime,
    end: NaiveDateTime,
}

async fn is_available(Query(auditorium): Query<AuditoriumQuery>) -> bool {
    todo!()
}

struct AuditoriumListQuery {
    building: IdOrName,
    floor: u8,
    start: NaiveDateTime,
    end: NaiveDateTime,
}
async fn get_availability(Query(auditoriums): Query<AuditoriumListQuery>) {
    todo!()
}
