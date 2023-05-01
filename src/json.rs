// a module dedicated to parsing json files
// use serde_json;
use serde;
use serde_json::json;
use serde_json::to_string;
use serde_json::to_string_pretty;
use serde_json::Value;
use std::fs::read_to_string;

use crate::json;

pub fn read_file(file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file_buffer = read_to_string(&file_name).unwrap();
    return Ok(file_buffer);
}

pub fn convert_to_json(json_text: &str) -> Value {
    // let mut json_obj = json!(json_text);
    // json_obj =
    // let json = to_string_pretty(&json_obj);
    let json_obj: Value = serde_json::from_str(json_text).expect("JSON was not well-formatted");

    // print!("{:?}", to_string_pretty(&json_text).unwrap());
    // println!("{}", serde_json::to_string_pretty(&json_text).unwrap());
    // println!("{:?}", &json_obj["chapterName"]);
    return json_obj;
}

pub fn read_json(file_path: &str) -> Value {
    let file_content = read_file(file_path).unwrap();

    return convert_to_json(&file_content);
}
