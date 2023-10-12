use serde::{Serialize, Deserialize};
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let client_id = std::env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID must be set.");
    println!("SPOTIFY_CLIENT_ID = {}", client_id);
}
