use reqwest::blocking;
use serde_json::Value;

pub fn search(query: &str, search_type: &str) -> Result<String, Box<dyn std::error::Error>> {
    // default values
    const BASE_URL: &str = "https://api.spotify.com/v1/search";
    let mut q = query.replace(" ", "%20");

    // making the get reqwest
    let response = reqwest::blocking::get(BASE_URL)?;
    let json_data = response.json::<serde_json::Value>()?;
    println!("{}", q);
    return Ok(q);
}
