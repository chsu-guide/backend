#![feature(iter_intersperse)]

use std::env;
use unitracker_chsu::request::*;
use unitracker_chsu::request::groups::get_groups;
use unitracker_chsu::request::schedule::{get_schedule, ScheduleRequestBuilder, ScheduleType};
use chrono::{Days, NaiveDate, Utc};
use unitracker_chsu::request::auditoriums::get_auditoriums;
use unitracker_chsu::request::teachers::get_teachers;
use unitracker_postgres::database;
use sqlx::query;
use dotenv;
use sqlx::postgres::PgPool;
use unitracker_chsu::request::auth::get_auth;
use unitracker_postgres::database::Database;

#[tokio::main]
async fn main() {
    dotenv::dotenv();

    let token = get_auth().await.unwrap();
    let bearer = &token.data;
    let url = env::var("DATABASE_URL").unwrap();
    let db = Database::new(&url).unwrap();
    let auds = get_auditoriums(bearer).await.unwrap();
    let first = &auds[0];
    db.insert_auditoriums(first.clone().into()).await.unwrap();
    let aud = db.select_auditoriums(first.id).await.unwrap();

    println!("{:?}", aud.unwrap())
}
