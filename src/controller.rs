use rocket::get;
use rocket::serde::json::Json;
use std::path::PathBuf;

pub mod todo;
use todo::*;

pub const PATH: &str = "timings.json";

#[get("/")]
pub fn index() -> Json<Vec<Todo>> {
    Json(get_all_todos(PATH))
}

#[get("/<task>/<time>/<date..>")]
pub fn create(task: &str, time: &str, date: PathBuf) -> Json<Vec<Todo>> {
    let data = Todo {
        do_by_time: utc_from(time, date),
        task: task.to_string(),
    };
    let contents = add_to_json(PATH, data, None);

    Json(contents)
}

#[get("/<id>")]
pub fn read(id: usize) -> Json<Todo> {
    get_todo(PATH, id)
}

#[get("/<id>/update/<task>/<time>/<date..>")]
pub fn update(id: usize, task: &str, time: &str, date: PathBuf) -> Json<Todo> {
    let data = Todo {
        do_by_time: utc_from(time, date),
        task: task.to_string(),
    };
    let mut json_contents = add_to_json(PATH, data, Some(id));

    Json(json_contents.remove(id))
}

#[get("/<id>/delete")]
pub fn delete(id: usize) -> Json<Vec<Todo>> {
    let mut json_contents = get_all_todos(PATH);
    json_contents.remove(id);
    rewrite_json(PATH, &json_contents);

    Json(json_contents)
}
