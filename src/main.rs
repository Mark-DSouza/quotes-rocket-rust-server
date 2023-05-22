#[macro_use]
extern crate rocket;

mod seed_data;

use crate::seed_data::seed_data;
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Quote {
    id: i32,
    author: String,
    content: String,
    category: String,
}

#[get("/random")]
fn get_random_quote() -> Json<Quote> {
    Json(Quote {
        id: 1,
        author: String::from("Henry van Dijk"),
        content: String::from("Use what talents you have - the woods will be very silent if no birds sang there except those that sang best."),
        category: String::from("Motivational"),
    })
}

#[get("/<id>")]
fn get_quote(id: i32) -> Json<Quote> {
    let quotes = seed_data();
    let quote: &Quote = quotes.iter().find(|&x| x.id == id).unwrap();
    Json(Quote {
        id,
        author: quote.author.clone(),
        content: quote.content.clone(),
        category: quote.category.clone(),
    })
}

#[get("/")]
fn get_all_quotes() -> Json<[Quote; 12]> {
    let quotes = seed_data();
    Json(quotes)
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index]).mount(
        "/quotes",
        routes![get_random_quote, get_quote, get_all_quotes],
    )
}
