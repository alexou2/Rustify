#![allow(unused)]
// use reqwest::Request;
use std::env;
use std::fs;
mod requester;

fn main() {
    println!("The host is running {}", env::consts::OS);

    let mut source_code = fs::read_to_string("src/main.rs");
    // println!("code: {:?}", read_file("text"));
    println!("{}", requester::test());
    fs::write("./er",  "source_code");
    fs::remove_file("./er");
}
