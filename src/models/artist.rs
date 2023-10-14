use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Artist {
    pub id: i32,
    pub stagename: String,
    pub appleurl: String,
    pub spotifyurl: String
}
