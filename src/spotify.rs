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


