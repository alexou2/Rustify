use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use reqwest::{blocking, header::EXPECT};
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;

const BASE_URL: &str = "https://api.spotify.com/v1/";

pub fn search(query: &str, search_type: &str, access_token: String) -> Result<Value, Box<dyn std::error::Error>> {
    // default values
    let method = "search";
    let mut q = query.replace(" ", "%20");
    let mut request_url = format!("{}{}?query={}&type={}", BASE_URL, method, q, search_type);
    let formatted_access_token = format!("Bearer {}", access_token);

    // making the get reqwest
    let client = reqwest::blocking::Client::new();
    let res = client.get(request_url)
    .header("Authorization", formatted_access_token)
    .send()?;
    // .headers("Authorization: Bearer","BQCIhY-r5P1wlhEawKthWQ4Y1nz9aJfbXEdR0p-OouOrpcNHpLU9Xe4zm2KM3FckX90HutEUBORrfnKfpe4Tp8uXO0cV662B8qfRLh4C1VaXBxvnhwJ3")
    // .await
    // ?;

    let json_data = res.json::<serde_json::Value>()?;
    println!("{:?}", json_data);
    return Ok(json_data);
}
