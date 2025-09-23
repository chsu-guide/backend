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

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    from: Option<String>,
    #[arg(short, long)]
    to: Option<String>,
}
const GROUP_ID: u64 = 1739582424505775711;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let from = match args.from {
        Some(ref string) => string.clone().parse()?,
        None => Utc::now().naive_local().date(),
    };
    let to = match args.from {
        Some(val) => val.clone().parse()?,
        None => Utc::now()
            .naive_local()
            .date()
            .checked_add_days(Days::new(7))
            .unwrap(),
    };
    let request = ScheduleRequestBuilder::new()
        .start(from)
        .end(to)
        .schedule_type(request::schedule::ScheduleType::Group(GROUP_ID))
        .build();
    let client = ChsuClient::new().await;
    let data = client.get_schedule(request).await.unwrap();
    // let table = generate_table(&client.unwrap());

    // println!("{table}");
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
