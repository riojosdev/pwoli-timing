use chrono::prelude::*;
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Read;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
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

    Utc.datetime_from_str(&timestamp, "%Y-%m-%d %H:%M:%S")
        .ok()
        .unwrap()
}

/// Writes the Todo data to a json file
pub fn add_to_json(path: &str, data: Todo, id: Option<usize>) -> Vec<Todo> {
    let mut json_contents: Vec<Todo> = get_all_todos(path);
    // Appending/inserting Todo data to the json file's contents
    match id {
        Some(index) => json_contents[index] = data,
        None => json_contents.push(data),
    };
    // Rewriting the entire data
    rewrite_json(path, &json_contents);

    json_contents
}

pub fn get_todo(path: &str, id: usize) -> Json<Todo> {
    let mut json_contents: Vec<Todo> = get_all_todos(path);

    Json(json_contents.remove(id))
}

pub fn get_all_todos(path: &str) -> Vec<Todo> {
    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(path)
        .unwrap();
    // Read json file contents
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    serde_json::from_str(&contents).unwrap()
}

pub fn rewrite_json(path: &str, data: &Vec<Todo>) {
    fs::write(path, serde_json::to_string(data).unwrap()).unwrap();
}
