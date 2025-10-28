#![allow(unused)] // A lot of population has to only be done once and making flags is annoying

use chrono::{Days, Utc};
use tokio;

use eyre::Result;
use tracing::info;
use unitracker_chsu::{
    ChsuClient,
    request::schedule::{ScheduleRequestBuilder, ScheduleType},
};
use unitracker_psql::database::Database;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("Starting up");
    let client = ChsuClient::new().await;
    // let database_url = env!("DATABASE_URL");
    let database =
        Database::new("postgres://unitracker:unitracker@127.0.0.1:3535/unitracker-db").unwrap();
    info!("Initialized client and database");
    // fill_buildings(&database, &client).await;
    // fill_teachers(&database, &client).await;
    // println!("Filled teachers\nFilling auditoriums");
    // fill_auditoriums(&database, &client).await;
    // println!("Filled auditoriums\nFilling disciplines");
    // fill_disciplines(&database, &client).await;
    // println!("Filled disciplines\nFilling groups");
    // fill_groups(&database, &client).await;
    println!("Filled groups\nFilling classes");
    fill_classes(&database, &client).await;
    Ok(())
}

async fn fill_teachers(database: &Database, client: &ChsuClient) {
    let api_teachers = client.get_teachers().await.unwrap();
    let db_teachers: Vec<_> = api_teachers
        .iter()
        .map(|f| unitracker_psql::models::teacher::Teacher::from(f.clone()))
        .collect();
    database.insert_teacher_many(&db_teachers).await.unwrap();
}

async fn fill_auditoriums(database: &Database, client: &ChsuClient) {
    info!("Filling auditoriums");
    let api_teachers = client.get_auditoriums().await.unwrap();
    database
        .insert_auditorium_many(&api_teachers)
        .await
        .unwrap();
    info!("Filled auditoriums");
}

async fn fill_buildings(database: &Database, client: &ChsuClient) {
    info!("Filling buildings");
    let api_teachers = client.get_buildings().await.unwrap();
    database.insert_building_many(&api_teachers).await.unwrap();
    info!("Filled buildings");
}

async fn fill_disciplines(database: &Database, client: &ChsuClient) {
    println!("Filling disciplines");
    let api_disciplines = client.get_disciplines().await.unwrap();
    println!("Got api response");
    let db_disciplines: Vec<_> = api_disciplines
        .iter()
        .map(|f| unitracker_psql::models::discipline::Discipline::from(f.clone()))
        .collect();
    println!("Converted to db structures");
    database
        .insert_discipline_many(&db_disciplines)
        .await
        .unwrap();
    println!("Filled disciplines");
}

async fn fill_groups(database: &Database, client: &ChsuClient) {
    let api_disciplines = client.get_groups().await.unwrap();
    database
        .initial_insert_groups_many(&api_disciplines)
        .await
        .unwrap();
}

#[tracing::instrument]
async fn fill_classes(database: &Database, client: &ChsuClient) {
    println!("starting class population");
    let api_classes = client
        .get_schedule(
            ScheduleRequestBuilder::new()
                .start(
                    Utc::now()
                        .checked_add_days(Days::new(8))
                        .unwrap()
                        .date_naive(),
                )
                .end(
                    Utc::now()
                        .checked_add_days(Days::new(22))
                        .unwrap()
                        .date_naive(),
                )
                .schedule_type(ScheduleType::Full)
                .build(),
        )
        .await
        .unwrap();
    println!("Got {} classes", api_classes.len());
    database.populate_classes(&api_classes).await.unwrap();
    println!("populated classes");
}
