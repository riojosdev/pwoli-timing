#[macro_use]
extern crate rocket;

use pwoli_timing::*;
use rocket::serde::json::Json;
use std::path::PathBuf;

pub const PATH: &str = "timings.json";

#[get("/<task>/<time>/<date..>")]
fn create(task: &str, time: &str, date: PathBuf) -> Json<Vec<Todo>> {
    let data = Todo {
        do_by_time: utc_from(time, date),
        task: task.to_string(),
    };
    let contents = add_to_json(PATH, data, None);

    Json(contents)
}

#[get("/<id>")]
fn read(id: usize) -> Json<Todo> {
    get_todo(PATH, id)
}

#[get("/<id>/update/<task>/<time>/<date..>")]
fn update(id: usize, task: &str, time: &str, date: PathBuf) -> Json<Todo> {
    let data = Todo {
        do_by_time: utc_from(time, date),
        task: task.to_string(),
    };
    let mut json_contents = add_to_json(PATH, data, Some(id));

    Json(json_contents.remove(id))
}

#[launch]
fn launch() -> _ {
    rocket::build().mount("/", routes![create, read, update])
}
