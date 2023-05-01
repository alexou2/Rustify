use std::collections::HashMap;
use crate::utils;


// pub fn read_creds() -> [&'static str; 2] {
// pub fn read_creds() -> (&'static str, &'static str) {
//     let file = File::open("src/spotify_creds.json").expect("failed to open file");
//     let reader = BufReader::new(file);

//     let json_data: Value = serde_json::from_reader(reader).expect("failed to parse JSON");

//     // let name = json_data["name"].as_str().unwrap();
//     let client_id = json_data["client_id"].as_str().unwrap();
//     let client_secret = json_data["client_secret"].as_str().unwrap();
//     // println!("{:?}\n{:?}", client_id, client_secret);

//     return (client_id, client_secret);
//     // return (client_id.to_string(), client_secret.to_string());
// }

pub fn auth() -> Result<String, Box<dyn std::error::Error>> {
    // let client_id = "your-client-id";
    // let client_secret = "your-client-secret";

    let [client_id, client_secret] = utils::return_credits();
    let auth_url = "https://accounts.spotify.com/api/token";

    let mut form_params = HashMap::new();
    form_params.insert("grant_type", "client_credentials");
    form_params.insert("client_id", client_id);
    form_params.insert("client_secret", client_secret);

    let client = reqwest::blocking::Client::new();
    let res = client
        .post(auth_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&form_params)
        .send()?;

    let body = res.json::<serde_json::Value>()?;
    let access_token = body["access_token"].as_str().unwrap();

    println!("Access Token: {}", &access_token);
    let token = access_token;
    return Ok(token.to_string());
}