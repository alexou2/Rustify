#![allow(unused)]
use std::env;
use std::fs;
// use reqwest::Error;
use reqwest::blocking::get;
use serde_json::json;
use serde_json::to_string_pretty;
use serde_json::Value;
mod spotify;
mod auth;

fn main() -> Result<(), reqwest::Error> {
    println!("The host is running {}", env::consts::OS);
// println!("{}", to_string_pretty(&spotify::login()).unwrap());
    // println!("{:?}", spotify::search("lol", "track").unwrap());
    Ok(())
}

fn print_type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}