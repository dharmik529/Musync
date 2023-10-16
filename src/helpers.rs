extern crate reqwest;
use reqwest::header;

pub fn get_access_token(clientid: String, clientsecret: String) -> Result<(), Box<dyn std::error::Error>> {
    let body: String = format!("grant_type=client_credentials&client_id={clientid}&client_secret={clientsecret}");
    
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());

    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let res = client.post("https://accounts.spotify.com/api/token")
        .headers(headers)
        .body(body)
        .send()?
        .text()?;
    println!("{}", res);

    Ok(())
}
