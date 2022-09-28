use chrono::prelude::*;
use rocket::serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Todo {
    #[serde(with = "my_date_format")]
    pub do_by_time: DateTime<Utc>,
    pub task: String,
}

mod my_date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S %Z";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

/// Converts time and date to Utc timestamp
pub fn utc_from(time: &str, date: PathBuf) -> DateTime<Utc> {
    // ! handle the errors for PathBuf and wrong time format
    let timestamp = format!(
        "{} {}",
        NaiveDate::parse_from_str(date.to_str().unwrap(), "%d\\%m\\%Y")
            .ok()
            .unwrap(),
        NaiveTime::parse_from_str(&format!("{}:00", time), "%H:%M:%S")
            .ok()
            .unwrap()
    );
    

    Utc
        .datetime_from_str(&timestamp, "%Y-%m-%d %H:%M:%S")
        .ok()
        .unwrap()
}
