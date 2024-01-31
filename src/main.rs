use reqwest;
use serde::Deserialize;
use clap::Parser;
use rand::{distributions::Alphanumeric, Rng};
use std::error::Error;
use std::thread;

use wallpaper;
use colored::*;
use terminal_log_symbols::{ERROR_SYMBOL, INFO_SYMBOL, SUCCESS_SYMBOL, WARNING_SYMBOL};

/// Simple program to fetch random image and set it as wallpaper 
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Query you want to search at https://wallhaven.cc
    #[arg(short, long, default_value_t = {"pixel".to_string()})]
    query: String,

    /// Search with colors [Don't add # in hex codes]
    #[arg(short, long, default_value_t = String::new())]
    color: String,

    /// Use API key [For searching NSFW images]
    #[arg(short, long, default_value_t = String::new())]
    key: String,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    data: Vec<Wallpaper>,
    meta: Meta
}

#[derive(Deserialize, Debug)]
struct Wallpaper {
    id: String,
    url: String,
    path: String,
}

#[derive(Deserialize, Debug)]
struct Meta {
    last_page: i32
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut api_url = format!("https://wallhaven.cc/api/v1/search?q={}&colors={}&sorting=random&seed={}", args.query, args.color, generate_seed());
    if !args.key.is_empty() { api_url = api_url+"&apikey="+&args.key; }

    println!("[{}] {} {}", INFO_SYMBOL.to_string().bright_green(), "Requesting API:".bright_green(), api_url.bright_white().bold());

    match reqwest::get(&api_url).await {
        Ok(response) => {
            if response.status().is_success() {
                let response_body = response.json::<ApiResponse>().await?;
                if response_body.data.is_empty() {

                    println!("[{}] {} - {} {} - {} {}", WARNING_SYMBOL.to_string().bright_yellow(), "No wallpaper was found".bright_yellow(), "Query:".bright_green(), args.query.bright_cyan(), "Color:".bright_green(), args.color.white().bold());

                } else {

                    thread::spawn(move || {
                        let mut rng = rand::thread_rng();
                        let random_index = rng.gen_range(0..response_body.data.len());
                        let random_wallpaper = &response_body.data[random_index];
                        println!("[{}] {} {}", SUCCESS_SYMBOL.to_string().bright_green(), "Found Wallpaper:".bright_blue(), random_wallpaper.path.to_string().bright_cyan().bold());
                        wallpaper::set_from_url(random_wallpaper.path.as_str()).unwrap();
                    }).join().expect("Thread panicked");

                    //println!("{:?}", wallpaper::get());
                }
            } else {
                println!("[{}] {} {}", ERROR_SYMBOL.to_string().bright_red().bold(), "Failed to fetch data:".bright_red(), response.status().to_string().bright_white().bold());
            }
        }
        Err(err) => {
            println!("[{}] {} {}", ERROR_SYMBOL.to_string().bright_red().bold(), "Request failed:".bright_red(), err.to_string().bright_white().bold());
        }
    }

    Ok(())
}


fn generate_seed() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect()
}