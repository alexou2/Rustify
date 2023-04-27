#![allow(unused)]
use std::env;
use std::fs;
// use reqwest::Error;
use reqwest::blocking::get;
use serde_json::json;
use serde_json::to_string_pretty;
use serde_json::Value;
mod auth;
mod spotify;

fn main() -> Result<(), reqwest::Error> {
    println!("The host is running {}", env::consts::OS);
    // println!("{}", to_string_pretty(&spotify::login()).unwrap());
    // println!("{:?}", spotify::search("lol", "track").unwrap());
    let mut access_token = auth::auth().unwrap();
    type_of(&access_token);
    spotify::search("lol", "artist", access_token);
    Ok(())
}

fn type_of<T>(_: &T){
    let type_of_var =  format!("{}", std::any::type_name::<T>());
    println!("{}", type_of_var)
}
