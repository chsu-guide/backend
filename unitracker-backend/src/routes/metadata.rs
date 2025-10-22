use std::sync::Arc;

use apply::Apply;
use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use serde::{Deserialize, Serialize};
use unitracker_server::context::Context;

#[derive(Debug, Serialize, Deserialize)]
struct Group {
    id: i64,
    name: String,
}

async fn get_groups(State(ctx): State<Arc<Context>>) -> Result<Json<Vec<Group>>, StatusCode> {
    let groups = ctx
        .database()
        .select_groups_all()
        .await
        .map_err(|_| StatusCode::IM_A_TEAPOT)?;
    groups
        .iter()
        .map(|g| Group {
            id: g.id,
            name: g.name.to_string(),
        })
        .collect::<Vec<_>>()
        .apply(|i| Ok(Json(i)))
}

#[derive(Debug, Serialize, Deserialize)]
struct Teacher {
    id: i64,
    full_name: String,
}

async fn get_teachers(State(ctx): State<Arc<Context>>) -> Result<Json<Vec<Teacher>>, StatusCode> {
    let teachers = ctx
        .database()
        .select_teacher_all()
        .await
        .map_err(|_| StatusCode::IM_A_TEAPOT)?;

    teachers
        .iter()
        .map(|t| Teacher {
            id: t.id,
            full_name: format!(
                "{} {} {}",
                t.last_name,
                t.first_name,
                t.middle_name.clone().unwrap_or_default()
            )
            .trim()
            .to_owned(),
        })
        .collect::<Vec<_>>()
        .apply(|t| Ok(Json(t)))
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildingList {
    pub id: i64,
    pub name: Box<str>,
    pub auditoriums: Vec<Auditorium>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Auditorium {
    pub id: i64,
    pub name: Box<str>,
    pub number: Box<str>,
}
async fn get_auditoriums(
    State(ctx): State<Arc<Context>>,
) -> Result<Json<Vec<BuildingList>>, StatusCode> {
    let builds = ctx
        .database()
        .select_buildings_with_auditoriums()
        .await
        .map_err(|_| StatusCode::IM_A_TEAPOT)?;

    let buildings = builds
        .iter()
        .map(|bwa| BuildingList {
            id: bwa.id,
            name: bwa.name.clone(),
            auditoriums: bwa
                .auditoriums
                .iter()
                .map(|aud| Auditorium {
                    id: aud.id,
                    name: aud.name.clone(),
                    number: aud.number.clone(),
                })
                .collect(),
        })
        .collect();
    Ok(Json(buildings))
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Discipline {
    pub id: i64,
    pub name: Box<str>,
}

async fn get_disciplines(
    State(ctx): State<Arc<Context>>,
) -> Result<Json<Vec<Discipline>>, StatusCode> {
    let disciplines = ctx
        .database()
        .select_discipline_all()
        .await
        .map_err(|_| StatusCode::IM_A_TEAPOT)?;
    Ok(Json(
        disciplines
            .iter()
            .map(|d| Discipline {
                id: d.id,
                name: d.name.clone(),
            })
            .collect(),
    ))
}

pub(crate) fn metadata_router() -> Router<Arc<Context>> {
    Router::new()
        .route("/metadata/groups", get(get_groups))
        .route("/metadata/teachers", get(get_teachers))
        .route("/metadata/auditoriums", get(get_auditoriums))
        .route("/metadata/disciplines", get(get_disciplines))
}
