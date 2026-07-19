use std::error::Error;
use reqwest::Client;

pub mod wallhaven;
pub mod unsplash;
pub mod pexels;
pub mod bing;

pub trait WallpaperSource {
    async fn fetch(&self, client: &Client, query: &str, color: &str, key: &str, orientation: &str) -> Result<String, Box<dyn Error>>;
}

pub enum WallpaperSourceImpl {
    Wallhaven(wallhaven::Wallhaven),
    Unsplash(unsplash::Unsplash),
    Pexels(pexels::Pexels),
    Bing(bing::Bing),
}

impl WallpaperSource for WallpaperSourceImpl {
    async fn fetch(&self, client: &Client, query: &str, color: &str, key: &str, orientation: &str) -> Result<String, Box<dyn Error>> {
        match self {
            Self::Wallhaven(s) => s.fetch(client, query, color, key, orientation).await,
            Self::Unsplash(s) => s.fetch(client, query, color, key, orientation).await,
            Self::Pexels(s) => s.fetch(client, query, color, key, orientation).await,
            Self::Bing(s) => s.fetch(client, query, color, key, orientation).await,
        }
    }
}
