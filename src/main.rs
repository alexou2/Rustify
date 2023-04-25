#![allow(unused)]
// use Reqwest;
use std::fs;
use std::env;

fn main() {
    println!("The host is running {}", env::consts::OS);

    let mut source_code = fs::read_to_string("src/main.rs");
    println!("code: {:?}", source_code)
}