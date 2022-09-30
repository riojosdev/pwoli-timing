#[macro_use]
extern crate rocket;

use pwoli_timing::controller::*;

#[launch]
fn launch() -> _ {
    rocket::build().mount("/", routes![index, create, read, update, delete])
}
