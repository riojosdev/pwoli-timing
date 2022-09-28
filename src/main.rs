#[macro_use]
extern crate rocket;

use pwoli_timing::*;
use rocket::serde::json::Json;
use std::path::PathBuf;

#[get("/<task>/<time>/<date..>")]
fn create(task: &str, time: &str, date: PathBuf) -> Json<Todo> {
    Json(Todo {
        do_by_time: utc_from(time, date),
        task: task.to_string(),
    })
}

#[get("/<id>")]
fn read(id: i8) -> Json<Vec<Todo>> {}

#[launch]
fn launch() -> _ {
    rocket::build().mount("/", routes![create])
}
