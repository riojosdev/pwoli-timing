#[macro_use]
extern crate rocket;

use pwoli_timing::*;
use rocket::serde::json::Json;
use std::path::PathBuf;

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
    get_todo("timings.json", id)
}

#[launch]
fn launch() -> _ {
    rocket::build().mount("/", routes![create, read])
}
