#![allow(unused)]
// use Reqwest;
// use std::fs;
use std::env;

fn main() {
    println!("Hello, world!");
    let osx = env::consts::OS;
    println!("The host is running {}", osx)
}
