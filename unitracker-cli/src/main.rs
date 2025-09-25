use chrono::{DateTime, Days, NaiveDate, Utc};
use clap::{Parser, arg, command};
use comfy_table::Table;
use tokio;

use eyre::Result;
use unitracker_chsu::{
    ChsuClient,
    model::schedule::Schedule,
    request::{self, schedule::ScheduleRequestBuilder},
};
use unitracker_psql::{
    database::Database,
    models::{auditorium::Auditorium, building::Building, group::Group, teacher::Teacher},
};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ChsuClient::new().await;
    let dbd =
        Database::new("postgres://unitracker:unitracker@127.0.0.1:3535/unitracker-db").unwrap();
    let data: Vec<unitracker_psql::models::teacher::Teacher> = client
        .get_teachers()
        .await
        .unwrap()
        .iter()
        .map(|b| Teacher::from(b.clone()))
        .collect();
    println!("{:?}", dbd.insert_teacher_many(&data).await);
    Ok(())
}

fn generate_table(schedule: &[Schedule]) -> Table {
    let mut binding = comfy_table::Table::new();
    let table = binding.set_header(comfy_table::Row::from(vec!["Время", "Пара"]));
    for entry in schedule {
        table.add_row(vec![
            format!(
                "{} {}-{}",
                entry.date_event, entry.start_time, entry.end_time
            ),
            entry.discipline.title.clone(),
        ]);
    }
    binding
}
