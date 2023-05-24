use diesel::prelude::Queryable;
use rocket::serde::Serialize;

#[derive(Serialize, Queryable)]
pub struct Quote {
    pub id: i32,
    pub author: String,
    pub content: String,
    pub category: String,
}
