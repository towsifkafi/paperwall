use std::error::Error;
use serde::Deserialize;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use reqwest::Client;
use rand::seq::SliceRandom;
use super::WallpaperSource;

#[derive(Deserialize, Debug)]
struct PexelsResponse {
    photos: Vec<PexelsPhoto>,
}

#[derive(Deserialize, Debug)]
struct PexelsPhoto {
    src: PexelsSrc,
}

#[derive(Deserialize, Debug)]
struct PexelsSrc {
    original: String,
}

pub struct Pexels;

impl WallpaperSource for Pexels {
    async fn fetch(&self, client: &Client, query: &str, color: &str, key: &str, orientation: &str) -> Result<String, Box<dyn Error>> {
        if key.is_empty() {
            return Err("Pexels requires an API key. Please provide it via config or CLI (--pexels-key).".into());
        }

        let mut api_url = if query.is_empty() || query == "pixel" {
            "https://api.pexels.com/v1/curated?per_page=50".to_string()
        } else {
            format!("https://api.pexels.com/v1/search?query={}&per_page=50", query)
        };

        if !(color.is_empty() || query.is_empty() || query == "pixel") {
            api_url = format!("{}&color={}", api_url, color);
        }

        if !orientation.is_empty() {
            let o = if orientation == "portrait" { "portrait" } else { "landscape" };
            api_url = format!("{}&orientation={}", api_url, o);
        }

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_str(key)?);

        let response = client.get(&api_url).headers(headers).send().await?;

        if response.status().is_success() {
            let response_body = response.json::<PexelsResponse>().await?;
            if response_body.photos.is_empty() {
                return Err("No wallpaper was found on Pexels".into());
            }

            let mut rng = rand::thread_rng();
            if let Some(photo) = response_body.photos.choose(&mut rng) {
                Ok(photo.src.original.clone())
            } else {
                Err("No wallpaper was found on Pexels".into())
            }
        } else {
            Err(format!("Failed to fetch data from Pexels: {}", response.status()).into())
        }
    }
}
