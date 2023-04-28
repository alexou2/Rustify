// #![allow(unused)]
use std::env;
// use std::fs;
// use reqwest::Error;
// use reqwest::blocking::get;
// use serde_json::json;
// use serde_json::to_string_pretty;
// use serde_json::Value;
mod auth;
mod spotify;
mod utils;

fn main() -> Result<(), reqwest::Error> {
    println!("The host is running {}", env::consts::OS);
    // println!("{}", to_string_pretty(&spotify::login()).unwrap());
    // println!("{:?}", spotify::search("lol", "track").unwrap());
    let access_token = auth::auth().unwrap();
    let json_data = spotify::search("lol", "artist", access_token);
    // print!("{}", access_token);
    Ok(())
}


