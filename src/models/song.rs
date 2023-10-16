use crate::models::artist::Artist;
use serde::Serialize;

#[derive(Queryable)]
pub struct Song{
    pub id: i32,
    pub title: String,
    pub artist: String,
    pub duration: String,
    pub appleurl: Option<String>,
    pub spotifyurl: Option<String>,
    pub transfered: bool,
}

impl Song {
    pub fn attach(self, artist: Artist) -> SongJson {
        SongJson {
            id: self.id,
            title: self.title,
            artist,
            duration: self.duration,
            appleurl: self.appleurl,
            spotifyurl: self.spotifyurl,
            transfered: self.transfered,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SongJson {
   pub id: i32,
    pub title: String,
    pub artist: String,
    pub duration: String,
    pub appleurl: Option<String>,
    pub spotifyurl: Option<String>,
    pub transfered: bool, 
}
