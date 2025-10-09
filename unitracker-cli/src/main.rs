use chrono::{Days, Utc};
use comfy_table::Table;
use tokio;

use eyre::Result;
use unitracker_chsu::{
    ChsuClient,
    model::schedule::Schedule,
    request::schedule::{ScheduleRequest, ScheduleRequestBuilder, ScheduleType},
};
use unitracker_psql::{
    database::Database,
    models::{
        class::{self, Class},
        teacher::Teacher,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ChsuClient::new().await;
    // let database_url = env!("DATABASE_URL");
    let database =
        Database::new("postgres://unitracker:unitracker@127.0.0.1:3535/unitracker-db").unwrap();
    println!("Initialized");
    // fill_buildings(&database, &client).await;
    // println!("Filled buildings\nFilling teachers");
    // fill_teachers(&database, &client).await;
    // println!("Filled teachers\nFilling auditoriums");
    // fill_auditoriums(&database, &client).await;
    // println!("Filled auditoriums\nFilling disciplines");
    // fill_disciplines(&database, &client).await;
    // println!("Filled disciplines\nFilling groups");
    // fill_groups(&database, &client).await;
    // println!("Filled groups\nFilling classes");
    fill_classes(&database, &client).await;
    println!("Filled classes");
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
    let api_teachers = client.get_auditoriums().await.unwrap();
    let db_teachers: Vec<_> = api_teachers
        .iter()
        .map(|f| unitracker_psql::models::auditorium::Auditorium::from(f.clone()))
        .collect();
    database.insert_auditorium_many(&db_teachers).await.unwrap();
}

async fn fill_buildings(database: &Database, client: &ChsuClient) {
    let api_teachers = client.get_buildings().await.unwrap();
    let db_teachers: Vec<_> = api_teachers
        .iter()
        .map(|f| unitracker_psql::models::building::Building::from(f.clone()))
        .collect();
    database.insert_building_many(&db_teachers).await.unwrap();
}

async fn fill_disciplines(database: &Database, client: &ChsuClient) {
    let api_disciplines = client.get_disciplines().await.unwrap();
    let db_disciplines: Vec<_> = api_disciplines
        .iter()
        .map(|f| unitracker_psql::models::discipline::Discipline::from(f.clone()))
        .collect();
    database
        .insert_discipline_many(&db_disciplines)
        .await
        .unwrap();
}

async fn fill_groups(database: &Database, client: &ChsuClient) {
    let api_disciplines = client.get_groups().await.unwrap();
    database
        .initial_insert_groups_many(&api_disciplines)
        .await
        .unwrap();
}

async fn fill_classes(database: &Database, client: &ChsuClient) {
    let api_classes = client
        .get_schedule(
            ScheduleRequestBuilder::new()
                .start(
                    Utc::now()
                        .checked_sub_days(Days::new(20))
                        .unwrap()
                        .date_naive(),
                )
                .end(
                    Utc::now()
                        .checked_add_days(Days::new(20))
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
}
