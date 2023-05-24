#[macro_use]
extern crate rocket;

use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::{Build, Rocket};

use self::models::*;
use self::schema::quotes::dsl::*;

mod database;
mod models;
mod schema;

#[get("/")]
fn index() -> Json<Vec<Quote>> {
    let connection = &mut database::establish_connection();
    quotes
        .load::<Quote>(connection)
        .map(Json)
        .expect("Error loading birds")
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index])
}
