
use chrono::Local;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::Table;
use unitracker_chsu::request::schedule::ScheduleType::{self};
use unitracker_chsu::request::schedule::{get_schedule, ScheduleRequestBuilder};
use unitracker_chsu::global_init;

const GROUP_ID: u64 = 1739582424505775711;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    global_init().await;
    let req = ScheduleRequestBuilder::new()
        .start(Local::now().date_naive())
        .end(Local::now().date_naive())
        .schedule_type(ScheduleType::Group(GROUP_ID))
        .build();

    let schedule = get_schedule(req).await?;

    let mut table = Table::new();
    table
        .set_header(vec!["Дата", "Время", "Предмет"])
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);
    for item in schedule {
        table.add_row(vec![
            item.date_event,
            format!("{}-{}", item.start_time, item.end_time),
            item.discipline.title,
        ]);
    }

    println!("{table}");

    Ok(())
}
