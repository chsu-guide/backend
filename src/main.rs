#![feature(iter_intersperse)]

use std::env;
use unitracker_chsu::request::buildings::get_buildings;
use sqlx::query;
use dotenv;
use sqlx::postgres::PgPool;
use unitracker_chsu::request::auditoriums::get_auditoriums;
use unitracker_chsu::request::auth::get_auth;
use unitracker_chsu::request::schedule::get_schedule;
use unitracker_chsu::request::teachers::get_teachers;
use unitracker_postgres::database::Database;

#[tokio::main]
async fn main() {
    dotenv::dotenv();

    let token = get_auth().await.unwrap();
    let bearer = &token.data;
    let url = env::var("DATABASE_URL").unwrap();
    let db = Database::new(&url).unwrap();
    let auds = get_schedule(bearer).await.unwrap();
    for i in &auds {
        let aud = db.insert_schedule(i.clone().into()).await.unwrap();
    }
    let output = db.select_schedule(auds[1].id).await.unwrap();

    println!("{:?}", output.unwrap())
}
