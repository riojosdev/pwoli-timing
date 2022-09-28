#[macro_use] extern crate rocket;
use std::path::PathBuf;
use chrono::prelude::*;
use rocket::serde::{Serialize, json::Json};

#[launch]
fn launch() -> _ {
    rocket::build().mount("/", routes![index])
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Todo {
    #[serde(with = "my_date_format")]
    do_by_time: DateTime<Utc>,
    task: String
}

mod my_date_format {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S %Z";

    pub fn serialize<S>(
        date: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[get("/<task>/<time>/<date..>")]
fn index(task: &str, time: &str, date: PathBuf) -> Json<Todo> {
    Json(Todo {
        do_by_time: utc_from(time, date),
        task: task.to_string()
    })
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