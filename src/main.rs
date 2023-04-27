#![allow(unused)]
use std::env;
use std::fs;
// use reqwest::Error;
use reqwest::blocking::get;
use serde_json::json;
use serde_json::Value;
mod spotify;

fn main() -> Result<(), reqwest::Error>  {
    println!("The host is running {}", env::consts::OS);

//     let response = reqwest::blocking::get("https://api.mangadex.org/manga")?;
    
//     let json_data = response.json::<serde_json::Value>()?;
  
// println!("{}", json_data["data"][0]["attributes"]);


println!("{:?}", spotify::search("lol", "song").unwrap());
Ok(())
}



fn print_type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}



fn fetch() -> Result<(), Box<dyn std::error::Error>> {
    let response = get("https://api.mangadex.org/manga")?.text()?;
    let json_response = json!(response);

    println!("json{}", json_response[0]);
    Ok(())
}
