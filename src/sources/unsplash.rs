use std::error::Error;
use serde::Deserialize;
use reqwest::Client;
use super::WallpaperSource;

#[derive(Deserialize, Debug)]
struct UnsplashResponse {
    urls: UnsplashUrls,
}

#[derive(Deserialize, Debug)]
struct UnsplashUrls {
    full: String,
}

pub struct Unsplash;

impl WallpaperSource for Unsplash {
    async fn fetch(&self, client: &Client, query: &str, color: &str, key: &str, orientation: &str) -> Result<String, Box<dyn Error>> {
        if key.is_empty() {
            return Err("Unsplash requires an API key to be provided in the config or CLI.".into());
        }

        let mut api_url = format!("https://api.unsplash.com/photos/random?client_id={}", key);
        if !query.is_empty() && query != "pixel" {
            api_url = format!("{}&query={}", api_url, query);
        }
        if !color.is_empty() {
            api_url = format!("{}&color={}", api_url, color);
        }
        if !orientation.is_empty() {
            let o = if orientation == "portrait" { "portrait" } else { "landscape" };
            api_url = format!("{}&orientation={}", api_url, o);
        }

        let response = client.get(&api_url).send().await?;
        if response.status().is_success() {
            let response_body = response.json::<UnsplashResponse>().await?;
            Ok(response_body.urls.full)
        } else {
            Err(format!("Failed to fetch data from Unsplash: {}", response.status()).into())
        }
    }
}
