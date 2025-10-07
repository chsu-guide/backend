use std::{collections::HashMap, io::IsTerminal};

use apply::*;
use axum::extract::{Query, State};
use chrono::Utc;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use unitracker_psql::models::class::Class;
use unitracker_types::IdOrName;

use crate::context::ApplicationContext;

#[derive(Deserialize)]
struct RemainingClassesQuery {
    discipline: IdOrName,
    group: IdOrName,
}
#[derive(Serialize)]
struct RemainingClasses {
    /// Map Type <-> Count
    passed: HashMap<Box<str>, usize>,
    /// Map Type <-> Count
    remaining: HashMap<Box<str>, usize>,
}
async fn count_remaining_classes(
    State(ctx): State<ApplicationContext>,
    Query(query): Query<RemainingClassesQuery>,
) -> Result<RemainingClasses, Box<dyn std::error::Error>> {
    let current_time = Utc::now().naive_local();
    let items = ctx
        .database()
        .select_class_by_name_and_group(query.group, query.discipline)
        .await?;
    let (passed, remaining) = items
        .into_iter()
        .partition(|c| c.end_time > current_time)
        .apply(lesson_type)
        .apply(into_counts);
    Ok(RemainingClasses { passed, remaining })
}

fn lesson_type(tuple: (Vec<Class>, Vec<Class>)) -> (Vec<Box<str>>, Vec<Box<str>>) {
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
