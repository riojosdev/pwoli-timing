#[macro_use] extern crate rocket;
use std::path::PathBuf;
use chrono::prelude::*;
// use rocket::serde::{Serialize, json::Json};

#[launch]
fn launch() -> _ {
    rocket::build().mount("/", routes![index])
}

// #[derive(Serialize)]
// #[serde(crate = "rocket::serde")]
// struct Todo {
//     do_by_time: Utc,
//     task: String
// }

// impl Todo {
//     fn new(do_by_time: Utc, task: String) -> Self {
//         Todo {
//             do_by_time,
//             task
//         }
//     }
// }

#[get("/<task>/<time>/<date..>")]
fn index(task: &str, time: &str, date: PathBuf) -> String {
    format!("{} {}", task, utc_from(time, date))
}

/// Converts time and date to Utc timestamp
fn utc_from(time: &str, date: PathBuf) -> DateTime<Utc> {
    // ! handle the errors for PathBuf and wrong time format
    let timestamp = format!(
        "{} {}", 
        NaiveDate::parse_from_str(&date.to_str().unwrap(), "%d\\%m\\%Y").ok().unwrap(), NaiveTime::parse_from_str(&format!("{}:00", time), "%H:%M:%S").ok().unwrap()
    );
    let utc_timestamp = Utc.datetime_from_str(&timestamp, "%Y-%m-%d %H:%M:%S").ok().unwrap();

    utc_timestamp
}