use std::{collections::HashMap, io::IsTerminal, sync::Arc};

use apply::*;
use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use chrono::Utc;
use itertools::Itertools;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use tracing::info;
use unitracker_psql::models::class::Class;
use unitracker_types::IdOrName;

use unitracker_server::context::Context;

#[derive(Deserialize)]
struct RemainingClassesQuery {
    discipline: IdOrName,
    group: IdOrName,
}
#[derive(Serialize, Deserialize)]
struct RemainingClasses {
    /// Map Type <-> Count
    passed: HashMap<String, usize>,
    /// Map Type <-> Count
    remaining: HashMap<String, usize>,
}

async fn count_remaining_classes(
    State(ctx): State<Arc<Context>>,
    Query(query): Query<RemainingClassesQuery>,
) -> Result<Json<RemainingClasses>, StatusCode> {
    let current_time = Utc::now().naive_local();
    info!("Current time: {}", current_time);
    let items = ctx
        .database()
        .select_class_by_name_and_group(query.group, query.discipline)
        .await
        .map_err(|e| StatusCode::BAD_REQUEST)?;
    info!("Found {} items", items.len());
    let (passed, remaining) = items
        .into_iter()
        .partition(|c| c.end_time > current_time)
        .apply(lesson_type)
        .apply(into_counts);
    Ok(Json(RemainingClasses { passed, remaining }))
}

fn lesson_type(tuple: (Vec<Class>, Vec<Class>)) -> (Vec<String>, Vec<String>) {
    (
        tuple.0.into_iter().map(Class::lesson_type).collect(),
        tuple.1.into_iter().map(Class::lesson_type).collect(),
    )
}

fn into_counts<T: Eq + std::hash::Hash>(
    tuple: (Vec<T>, Vec<T>),
) -> (HashMap<T, usize>, HashMap<T, usize>) {
    (tuple.0.into_iter().counts(), tuple.1.into_iter().counts())
}

pub fn statistics_router() -> Router<Arc<Context>> {
    Router::new().route("/statistics/remaining", get(count_remaining_classes))
}
