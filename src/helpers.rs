extern crate reqwest;
use reqwest::header;

fn get_access_token() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());

    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let res = client.post("https://accounts.spotify.com/api/token")
        .headers(headers)
        .body("grant_type=client_credentials&client_id=your-client-id&client_secret=your-client-secret")
        .send()?
        .text()?;
    println!("{}", res);

    Ok(())
}
