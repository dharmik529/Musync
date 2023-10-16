extern crate reqwest;
use reqwest::header;

pub fn get_access_token() -> Result<(), Box<dyn std::error::Error>> {
let clientid: &str = "0cca86ed406f4f548686852611b6789c";
    let clientsecret: &str = "6d4daf9484a54d7ab6f442ce704c7d0e";
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
