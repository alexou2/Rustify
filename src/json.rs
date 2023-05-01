// a module dedicated to parsing json files
// use serde_json;
use std::fs::read_to_string;
use serde;
use serde_json::Value;

pub fn read_file(file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file_buffer = read_to_string(&file_name).unwrap();
    return Ok(file_buffer);
}
