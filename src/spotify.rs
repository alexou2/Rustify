use reqwest::{blocking, header::EXPECT};
use serde_json::Value;
use serde_json::json;

const BASE_URL: &str = "https://api.spotify.com/v1/";
pub fn search(query: &str, search_type: &str) -> Result<Value, Box<dyn std::error::Error>> {
    // default values
    let method = "search";
    let mut q = query.replace(" ", "%20");
    let mut request_url = format!("{}{}?query={}&type={}", BASE_URL, method, q, search_type);

    // let access_token = login();

    // making the get reqwest
    let response = reqwest::blocking::get(&request_url)?;
    let json_data = response.json::<serde_json::Value>()?;
    println!("{:?}", request_url);
    return Ok(json_data);
}

pub fn login() -> &'static str {
    const LOGIN_URL: &str = "https://accounts.spotify.com/api/token";
    const CONTENT_TYPE:&str = "Content-Type: application/x-www-form-urlencoded";

    let mut acces_token: &str = "";
    let client = reqwest::blocking::Client::new();
    let mut response = client.post(LOGIN_URL).body(CONTENT_TYPE).send().expect("err");
    let json_data = response.json::<serde_json::Value>();
    // print!("{:?}",json_data["url"]);
    return "acces_token";
}
