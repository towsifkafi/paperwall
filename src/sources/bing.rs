use std::error::Error;
use serde::Deserialize;
use reqwest::Client;
use super::WallpaperSource;

#[derive(Deserialize, Debug)]
struct BingResponse {
    images: Vec<BingImage>,
}

#[derive(Deserialize, Debug)]
struct BingImage {
    url: String,
}

pub struct Bing;

impl WallpaperSource for Bing {
    async fn fetch(&self, client: &Client, _query: &str, _color: &str, _key: &str, _orientation: &str) -> Result<String, Box<dyn Error>> {
        let api_url = "https://www.bing.com/HPImageArchive.aspx?format=js&n=1";
        
        let response = client.get(api_url).send().await?;

        if response.status().is_success() {
            let response_body = response.json::<BingResponse>().await?;
            if response_body.images.is_empty() {
                return Err("No wallpaper was found on Bing".into());
            }

            let relative_url = &response_body.images[0].url;
            let full_url = format!("https://www.bing.com{}", relative_url);
            Ok(full_url)
        } else {
            Err(format!("Failed to fetch data from Bing: {}", response.status()).into())
        }
    }
}
