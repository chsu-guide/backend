use axum::{
    Router,
    extract::{Query, State},
    routing::get,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use unitracker_types::IdOrName;

use crate::context::{ApplicationContext, create_context};

#[derive(Deserialize)]
struct AuditoriumQuery {
    auditorium: IdOrName,
    #[serde(with = "chrono::naive::serde::ts_seconds")]
    start: NaiveDateTime,
    #[serde(with = "chrono::naive::serde::ts_seconds")]
    end: NaiveDateTime,
}

#[derive(Serialize)]
enum AvailabilityResponse {
    Free,
    Busy(Vec<BusyRange>),
}
#[derive(Serialize)]
struct BusyRange {
    start: NaiveDateTime,
    end: NaiveDateTime,
}
async fn is_available(
    State(ctx): State<ApplicationContext>,
    Query(auditorium): Query<AuditoriumQuery>,
) -> Result<AvailabilityResponse, Box<dyn std::error::Error>> {
    match auditorium.auditorium {
        IdOrName::Id(_) => todo!(),
        IdOrName::Name(name) => {
            let auds = ctx
                .database()
                .auditorium_is_available(&name, auditorium.start, auditorium.end)
                .await?;
            if auds.is_empty() {
                Ok(AvailabilityResponse::Free)
            } else {
                Ok(AvailabilityResponse::Busy(
                    auds.iter()
                        .map(|a| BusyRange {
                            start: a.start_time,
                            end: a.end_time,
                        })
                        .collect::<Vec<_>>(),
                ))
            }
        }
    }
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
// pub fn get_auditorium_router() -> Router {
//     Router::new()
//         .route("/auditorium/available", get(is_available))
//         .with_state(create_context(
//             "postgres://unitracker:unitracker@127.0.0.1:3535/unitracker-db",
//         ))
// }
