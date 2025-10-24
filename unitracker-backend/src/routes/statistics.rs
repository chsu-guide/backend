use std::{collections::HashMap, fmt::Display, sync::Arc};

use apply::*;
use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    routing::get,
};
use chrono::Utc;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tracing::debug;
use unitracker_psql::models::class::Class;
use unitracker_types::IdOrName;

use unitracker_server::context::Context;

#[derive(Deserialize, Debug)]
struct RemainingClassesQuery {
    discipline: IdOrName,
    group: IdOrName,
}
impl Display for RemainingClassesQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Discipline: {:?}, group: {:?}",
            self.discipline, self.group
        )
    }
}
#[derive(Serialize, Deserialize, Debug)]
struct RemainingClasses {
    /// Map Type <-> Count
    passed: HashMap<String, usize>,
    /// Map Type <-> Count
    remaining: HashMap<String, usize>,
}

#[tracing::instrument(skip(ctx), fields(query = %query))]
async fn count_remaining_classes(
    State(ctx): State<Arc<Context>>,
    Query(query): Query<RemainingClassesQuery>,
) -> Result<Json<RemainingClasses>, StatusCode> {
    let current_time = Utc::now().naive_local();
    let items = ctx
        .database()
        .select_class_by_group_and_discipline(query.group, query.discipline)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    debug!("Found {} items", items.len());
    let (passed, remaining) = items
        .into_iter()
        .partition(|c| c.end_time > current_time)
        .apply(lesson_type)
        .apply(into_counts);
    Ok(Json(RemainingClasses { passed, remaining }))
}

/// Map two lists of classes to two lists of lesson types
fn lesson_type(tuple: (Vec<Class>, Vec<Class>)) -> (Vec<String>, Vec<String>) {
    (
        tuple.0.into_iter().map(Class::lesson_type).collect(),
        tuple.1.into_iter().map(Class::lesson_type).collect(),
    )
}

/// Map two vectors of Eq + Hash into two HashMaps of item counts
fn into_counts<T: Eq + std::hash::Hash>(
    tuple: (Vec<T>, Vec<T>),
) -> (HashMap<T, usize>, HashMap<T, usize>) {
    (tuple.0.into_iter().counts(), tuple.1.into_iter().counts())
}

/// Maps /statistics/remaining
pub fn statistics_router() -> Router<Arc<Context>> {
    Router::new().route("/statistics/remaining", get(count_remaining_classes))
}
