use serde::{Serialize, Deserialize};
use dotenv::dotenv;

#[derive(Serialize, Deserialize, Debug)]
struct SpotifyKeys {
    spotify_client_id : String,
    spotify_client_secret : String,
    spotify_redirect_url : String,
    spotify_access_token : Option<String>,
}

fn main() {
    dotenv().ok();
    let spotify_keys = get_access_tocken();
    let serialized = serde_json::to_string(&spotify_keys).unwrap();
    println!("serialized = {}", serialized);
}

fn get_access_tocken() ->  SpotifyKeys {
    
    let spotify_keys = SpotifyKeys {
        spotify_client_id : std::env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID must be set."),
        spotify_client_secret : std::env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET must be set."),
        spotify_redirect_url : std::env::var("SPOTIFY_REDIRECT_URI").expect("SPOTIFY_REDIRECT_URI must be set."),
        spotify_access_token : None
    };

    return spotify_keys;
}
