use serde::{Serialize, Deserialize};
use dotenv::dotenv;

mod spotify_helper;
pub mod models;

#[derive(Serialize, Deserialize, Debug)]
struct SpotifyKeys {
    spotify_client_id : String,
    spotify_client_secret : String,
    spotify_redirect_url : String,
    spotify_access_token : Option<String>,
}

fn main() {
    dotenv().ok();
    let spotify_keys = toSK();

    spotify_keys.spotify_access_token = spotify_helper::get_access_token(spotify_keys.spotify_client_id, spotify_keys.spotify_client_secret);  
}

fn toSK() ->  SpotifyKeys {
    
    let spotify_keys = SpotifyKeys {
        spotify_client_id : std::env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID must be set."),
        spotify_client_secret : std::env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET must be set."),
        spotify_redirect_url : std::env::var("SPOTIFY_REDIRECT_URI").expect("SPOTIFY_REDIRECT_URI must be set."),
        spotify_access_token : None
    };

    return spotify_keys;
}
