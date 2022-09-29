#[macro_use]
extern crate rocket;

use pwoli_timing::*;
use rocket::serde::json::Json;
use std::path::PathBuf;

use std::fs::OpenOptions;
use std::io::Read;

#[get("/<task>/<time>/<date..>")]
fn create(task: &str, time: &str, date: PathBuf) -> Json<Vec<Todo>> {
    let data = Todo {
        do_by_time: utc_from(time, date),
        task: task.to_string(),
    };

    let contents = add_to_json("timings.json", data);
    Json(contents)
}

#[get("/<id>")]
fn read(id: usize) -> Json<Todo> {
    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open("timings.json")
        .unwrap();

    // Read json file contents
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut json_contents: Vec<Todo> = serde_json::from_str(&contents).unwrap();

    let data = Json(json_contents.remove(id));
    data
}

#[launch]
fn launch() -> _ {
    rocket::build().mount("/", routes![create, read])
}
