#![allow(unused)] // A lot of population has to only be done once and making flags is annoying

use std::time::Duration;

use chrono::{Days, Utc};
use indicatif::{ProgressBar, ProgressStyle};
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
    database.migrate().await?;
    fill_buildings(&database, &client).await;
    fill_teachers(&database, &client).await;
    // println!("Filled teachers\nFilling auditoriums");
    fill_auditoriums(&database, &client).await;
    // println!("Filled auditoriums\nFilling disciplines");
    fill_disciplines(&database, &client).await;
    // println!("Filled disciplines\nFilling groups");
    fill_groups(&database, &client).await;
    fill_classes(&database, &client).await;
    Ok(())
}
fn new_spinner(idx: usize) -> ProgressBar {
    let spinner = ProgressBar::new_spinner().with_style(
        ProgressStyle::with_template(&format!("[{idx} / 6] {{spinner}} {{msg}} {{elapsed}}"))
            .unwrap()
            .tick_chars("⣾⣽⣻⢿⡿⣟⣯⣷"),
    );
    spinner.enable_steady_tick(Duration::from_millis(200));
    spinner
}

async fn fill_buildings(database: &Database, client: &ChsuClient) {
    let spinner = new_spinner(1).with_message("Fetching buildings");
    let api_teachers = client.get_buildings().await.unwrap();
    spinner.set_message("Inserting buildings into the database");
    database.insert_building_many(&api_teachers).await.unwrap();
    spinner.finish_with_message("Buildings filled!");
}
async fn fill_teachers(database: &Database, client: &ChsuClient) {
    let spinner = new_spinner(2).with_message("Fetching teachers");
    let api_teachers = client.get_teachers().await.unwrap();
    spinner.set_message("Inserting teachers into the database");
    let db_teachers: Vec<_> = api_teachers
        .iter()
        .map(|f| unitracker_psql::models::teacher::Teacher::from(f.clone()))
        .collect();
    database.insert_teacher_many(&db_teachers).await.unwrap();
    spinner.finish_with_message("Teachers filled!");
}

async fn fill_auditoriums(database: &Database, client: &ChsuClient) {
    let spinner = new_spinner(3).with_message("Fetching auditoriums");
    let api_teachers = client.get_auditoriums().await.unwrap();
    spinner.set_message("Inserting auditoriums into the database");
    database
        .insert_auditorium_many(&api_teachers)
        .await
        .unwrap();
    spinner.finish_with_message("Auditoriums filled!");
}

async fn fill_disciplines(database: &Database, client: &ChsuClient) {
    let spinner = new_spinner(4).with_message("Fetching disciplines");
    let api_disciplines = client.get_disciplines().await.unwrap();
    let db_disciplines: Vec<_> = api_disciplines
        .iter()
        .map(|f| unitracker_psql::models::discipline::Discipline::from(f.clone()))
        .collect();
    spinner.set_message("Inserting disciplines into the database");
    database
        .insert_discipline_many(&db_disciplines)
        .await
        .unwrap();
    spinner.finish_with_message("Disciplines filled!");
}

async fn fill_groups(database: &Database, client: &ChsuClient) {
    let spinner = new_spinner(5).with_message("Fetching groups");
    let api_disciplines = client.get_groups().await.unwrap();
    spinner.set_message("Inserting groups into the database");
    database
        .initial_insert_groups_many(&api_disciplines)
        .await
        .unwrap();
    spinner.finish_with_message("Groups filled!");
}

#[tracing::instrument]
async fn fill_classes(database: &Database, client: &ChsuClient) {
    let spinner = new_spinner(6).with_message("Fetching classes");
    let api_classes = client
        .get_schedule(
            ScheduleRequestBuilder::new()
                .start(
                    Utc::now()
                        .checked_sub_days(Days::new(60))
                        .unwrap()
                        .date_naive(),
                )
                .end(
                    Utc::now()
                        .checked_add_days(Days::new(45))
                        .unwrap()
                        .date_naive(),
                )
                .schedule_type(ScheduleType::Full)
                .build(),
        )
        .await
        .unwrap();
    spinner.set_message("Inserting classes into the database");
    database.populate_classes(&api_classes).await.unwrap();
    spinner.finish_with_message("Classes filled!");
}
