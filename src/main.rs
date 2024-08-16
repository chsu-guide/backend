mod update;

use std::env;
use chrono::{NaiveDate, TimeZone, Utc};
use dotenv;
use reqwest::ClientBuilder;
use unitracker_chsu::request::*;
use unitracker_chsu::request::auditoriums::get_auditoriums;
use unitracker_chsu::request::auth::get_auth;
use unitracker_postgres::database::Database;

#[tokio::main]
async fn main() -> Result<(), RequestErrors> {
    dotenv::dotenv().expect("TODO: panic message");

    let mut client = ClientBuilder::new().user_agent("").build()?;

    let auth = get_auth(&mut client).await.unwrap();

    let db_url = env::var("DATABASE_URL").unwrap();

    let db = Database::new(&db_url).unwrap();

    let auditoriums = get_auditoriums(&mut client, &auth.data).await.unwrap();

    let funky = auditoriums.iter().count();
    println!("{funky:?}");

    let max_id = auditoriums.clone().iter().map(|aud| aud.id).max().unwrap();
    println!("{max_id}");

    db.insert_auditorium_many(auditoriums).await.unwrap();
    //
    // for aud in auditoriums {
    //     db.insert_auditorium(aud).await.unwrap();
    // }

    let db_auds = db.select_auditorium(max_id).await.unwrap();

    println!("{db_auds:?}");

    Ok(())

}
