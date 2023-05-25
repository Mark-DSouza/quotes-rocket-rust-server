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

    let quote_ids = quotes
        .select(id)
        .load::<i32>(connection)
        .expect("Error loading quote IDs");

    let range = 0..quote_ids.len();
    let random_index_in_range = rand::thread_rng().gen_range(range);
    let random_id = quote_ids[random_index_in_range];

    let quote = quotes
        .find(random_id)
        .first::<Quote>(connection)
        .expect("Error finding quote with random ID");

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

#[get("/<quote_id>")]
fn get_quote(quote_id: i32) -> Json<Quote> {
    let connection = &mut database::establish_connection();

    let quote = quotes
        .find(quote_id)
        .first::<Quote>(connection)
        .expect("Error finding quote with random ID");

    Json(Quote {
        id: quote_id.clone(),
        author: quote.author.clone(),
        content: quote.content.clone(),
        category: quote.category.clone(),
    })
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount(
        "/api/v1/quotes",
        routes![index, get_random_quote, get_quote],
    )
}
