#[macro_use]
extern crate rocket;

mod seed_data;

use crate::seed_data::seed_data;
use rand::Rng;
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
    let quotes = seed_data();

    let random_number_in_quotes_range = rand::thread_rng().gen_range(0..quotes.len() as i32);
    let random_id = random_number_in_quotes_range + 1;

    let quote: &Quote = quotes.iter().find(|&x| x.id == random_id).unwrap();

    Json(Quote {
        id: quote.id.clone(),
        author: quote.author.clone(),
        content: quote.content.clone(),
        category: quote.category.clone(),
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
fn get_all_quotes() -> Json<Vec<Quote>> {
    let quotes = seed_data();
    Json(quotes)
}

#[post("/", data = "<quote>")]
fn create_quote(quote: Json<Quote>) -> Json<Quote> {
    quote
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index]).mount(
        "/quotes",
        routes![get_random_quote, get_quote, get_all_quotes, create_quote],
    )
}
