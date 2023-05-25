#[macro_use]
extern crate rocket;

use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::{Build, Rocket};

use rand::Rng;

use self::models::*;
use self::schema::quotes::dsl::*;

mod database;
mod models;
mod schema;

#[get("/random")]
fn get_random_quote() -> Json<Quote> {
    let connection = &mut database::establish_connection();

    let all_quotes = quotes
        .load::<Quote>(connection)
        .map(Json)
        .expect("Error loading birds");

    let range = 0..all_quotes.len() as i32;
    let random_number_in_range = rand::thread_rng().gen_range(range);
    let random_id = random_number_in_range + 1;

    let quote: &Quote = all_quotes.iter().find(|&x| x.id == random_id).unwrap();

    Json(Quote {
        id: quote.id.clone(),
        author: quote.author.clone(),
        content: quote.content.clone(),
        category: quote.category.clone(),
    })
}

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
    rocket::build().mount("/", routes![index, get_random_quote])
}
