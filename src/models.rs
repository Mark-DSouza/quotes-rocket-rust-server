use diesel::prelude::Queryable;
use diesel::Insertable;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;

use crate::schema::quotes;

#[derive(Serialize, Queryable)]
pub struct Quote {
    pub id: i32,
    pub author: String,
    pub content: String,
    pub category: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = quotes)]
pub struct QuoteInput {
    pub author: String,
    pub content: String,
    pub category: String,
}
