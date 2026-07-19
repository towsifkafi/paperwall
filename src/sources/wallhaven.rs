use std::error::Error;
use rand::{distributions::Alphanumeric, Rng};
use rand::seq::SliceRandom;
use serde::Deserialize;
use reqwest::Client;
use super::WallpaperSource;

#[derive(Deserialize, Debug)]
struct ApiResponse {
    data: Vec<Wallpaper>,
}

#[derive(Deserialize, Debug)]
struct Wallpaper {
    path: String,
}

pub struct Wallhaven;

impl WallpaperSource for Wallhaven {
    async fn fetch(&self, client: &Client, query: &str, color: &str, key: &str, orientation: &str) -> Result<String, Box<dyn Error>> {
        let ratios = if orientation == "portrait" { "9x16,10x16" } else { "16x9,16x10,21x9,32x9,3:2" };
        let mut api_url = format!("https://wallhaven.cc/api/v1/search?q={}&colors={}&sorting=random&seed={}&ratios={}", query, color, generate_seed(), ratios);
        if !key.is_empty() { api_url = api_url + "&apikey=" + key; }

        let response = client.get(&api_url).send().await?;
        if response.status().is_success() {
            let response_body = response.json::<ApiResponse>().await?;
            if response_body.data.is_empty() {
                return Err("No wallpaper was found".into());
            }

            let mut rng = rand::thread_rng();
            if let Some(wallpaper) = response_body.data.choose(&mut rng) {
                Ok(wallpaper.path.clone())
            } else {
                Err("No wallpaper was found".into())
            }
        } else {
            Err(format!("Failed to fetch data: {}", response.status()).into())
        }
    }
}

fn generate_seed() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect()
}
