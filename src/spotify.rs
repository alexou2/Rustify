use reqwest::blocking;

pub fn search(query: &str, args: &str) -> &'static str {
    const base_url: &str = "https://api.spotify.com/v1/search";
    // making the get reqwest
    let response = reqwest::blocking::get("https://api.mangadex.org/manga")?;
    let json_data = response.json::<serde_json::Value>()?;

    return "works";
}
