use serde::{ Serialize };
use diesel::Queryable;

#[derive(Queryable, Serialize)]
pub struct Artist {
    pub id: i32,
    pub stagename: String,
    pub appleurl: String,
    pub spotifyurl: String
}
