
use serde::Deserialize;
#[macro_use]
mod logger;
use clap::{Parser, Subcommand, CommandFactory};
use clap_complete::{generate, Shell};
use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;
use std::io;

use wallpape_rs as wallpaper;
use colored::*;

use indicatif::{ProgressBar, ProgressStyle};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Query you want to search at https://wallhaven.cc [default: pixel]
    #[arg(short, long)]
    query: Option<String>,
    
    /// Search with colors [Don't add # in hex codes]
    #[arg(short, long)]
    color: Option<String>,

    /// Use API key for Wallhaven
    #[arg(long)]
    wallhaven_key: Option<String>,

    /// Use API key for Unsplash
    #[arg(long)]
    unsplash_key: Option<String>,

    /// Use API key for Pexels
    #[arg(long)]
    pexels_key: Option<String>,

    /// Location to save the wallpaper
    #[arg(long)]
    download_location: Option<String>,

    /// Search for portrait or landscape wallpapers (default: landscape)
    #[arg(long)]
    orientation: Option<String>,

    /// Path to a custom config file
    #[arg(long)]
    config: Option<String>,

    /// Source to fetch wallpaper from (wallhaven, unsplash, pexels, bing)
    #[arg(short, long)]
    source: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate shell completions
    Completion {
        /// The shell to generate the completions for
        #[arg(value_enum)]
        shell: Shell,
    },
    /// Local history management
    History {
        #[command(subcommand)]
        action: HistoryAction,
    },
}

#[derive(Subcommand, Debug)]
enum HistoryAction {
    /// List history
    List,
    /// Revert to previous wallpaper
    Revert {
        /// Optional index (1 = previous, 2 = 2 wallpapers ago)
        #[arg(default_value = "1")]
        index: usize,
    },
    /// Clear history
    Clear,
}

#[derive(Deserialize, Debug, Default)]
struct ApiKeys {
    wallhaven: Option<String>,
    unsplash: Option<String>,
    pexels: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
struct Config {
    query: Option<String>,
    color: Option<String>,
    api_keys: Option<ApiKeys>,
    save_wallpaper: Option<bool>,
    download_location: Option<String>,
    orientation: Option<String>,
    source: Option<String>,
}

impl Config {
    fn load(custom_path: Option<&String>) -> Self {
        let path = if let Some(c) = custom_path {
            Some(PathBuf::from(c))
        } else {
            dirs::config_dir().map(|d| d.join("paperwall").join("config.yml"))
        };

        if let Some(p) = &path {
            if p.exists() {
                if let Ok(content) = std::fs::read_to_string(p) {
                    if let Ok(config) = serde_yaml::from_str::<Config>(&content) {
                        info!("{} {}", "Loaded config from:".bright_green(), p.display().to_string().bright_white().bold());
                        return config;
                    } else {
                        error!("{}", "Failed to parse config.yml".bright_red());
                    }
                }
            } else if custom_path.is_some() {
                warn!("{} {}", "Provided config file does not exist:".bright_yellow(), p.display().to_string().bright_white().bold());
            }
        }
        Config::default()
    }
}

mod history;
mod sources;
use sources::{WallpaperSource, wallhaven::Wallhaven, unsplash::Unsplash, pexels::Pexels, bing::Bing};

fn create_spinner(msg: String) -> ProgressBar {
    let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ");
    let pb = ProgressBar::new_spinner();
    pb.set_style(spinner_style);
    pb.set_message(msg);
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}

async fn apply_wallpaper(source: &str, url: &str, path: Option<PathBuf>) {
    let pb_apply = create_spinner("Applying wallpaper...".bright_blue().to_string());
    
    if let Some(p) = &path {
        let p_str = p.to_str().unwrap().to_owned();
        tokio::task::spawn_blocking(move || {
            wallpaper::set_from_path(&p_str).unwrap();
        }).await.expect("Task panicked");
    } else {
        let url_clone = url.to_owned();
        tokio::task::spawn_blocking(move || {
            wallpaper::set_from_url(&url_clone).unwrap();
        }).await.expect("Task panicked");
    }
    
    pb_apply.finish_and_clear();
    success!("{}", "Wallpaper set successfully!".bright_green().bold());
    history::History::add(source, url, path);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if let Some(cmd) = &args.command {
        match cmd {
            Commands::Completion { shell } => {
                let mut cmd = Args::command();
                generate(*shell, &mut cmd, "paperwall", &mut io::stdout());
                return Ok(());
            },
            Commands::History { action } => {
                match action {
                    HistoryAction::List => {
                        history::History::list();
                    },
                    HistoryAction::Clear => {
                        history::History::clear();
                    },
                    HistoryAction::Revert { index } => {
                        if let Some(entry) = history::History::get(*index) {
                            if let Some(p) = &entry.local_path {
                                if !p.exists() {
                                    apply_wallpaper(&entry.source, &entry.image_url, None).await;
                                    return Ok(());
                                }
                            }
                            apply_wallpaper(&entry.source, &entry.image_url, entry.local_path).await;
                        } else {
                            error!("{}", "History entry not found.".bright_red());
                        }
                    }
                }
                return Ok(());
            }
        }
    }

    let mut save_location: Option<PathBuf> = None;
    let mut should_save_wallpaper = false;

    let mut final_query = "pixel".to_string();
    let mut final_color = String::new();
    let mut final_orientation = "landscape".to_string();
    let mut final_source = "wallhaven".to_string();

    let mut config_wallhaven_key = String::new();
    let mut config_unsplash_key = String::new();
    let mut config_pexels_key = String::new();

    let config = Config::load(args.config.as_ref());
    if let Some(q) = config.query { final_query = q; }
    if let Some(c) = config.color { final_color = c; }
    if let Some(keys) = config.api_keys {
        if let Some(k) = keys.wallhaven { config_wallhaven_key = k; }
        if let Some(k) = keys.unsplash { config_unsplash_key = k; }
        if let Some(k) = keys.pexels { config_pexels_key = k; }
    }
    if let Some(s) = config.source { final_source = s; }
    if let Some(o) = config.orientation { final_orientation = o; }
    if let Some(sw) = config.save_wallpaper { should_save_wallpaper = sw; }
    if let Some(loc) = config.download_location { 
        let path = PathBuf::from(loc);
        if path.exists() {
            save_location = Some(path); 
        } else {
            warn!("{} {}", "Save location does not exist:".bright_yellow(), path.display().to_string().bright_white().bold());
        }
    }

    if let Some(q) = args.query { final_query = q; }
    if let Some(c) = args.color { final_color = c; }
    if let Some(s) = args.source { final_source = s; }
    if let Some(o) = args.orientation { final_orientation = o; }

    let final_key = match final_source.as_str() {
        "unsplash" => args.unsplash_key.unwrap_or(config_unsplash_key),
        "pexels" => args.pexels_key.unwrap_or(config_pexels_key),
        _ => args.wallhaven_key.unwrap_or(config_wallhaven_key),
    };

    if let Some(loc) = &args.download_location {
        let path = PathBuf::from(loc);
        if path.exists() {
            save_location = Some(path);
            should_save_wallpaper = true;
        } else {
            warn!("{} {}", "CLI save location does not exist:".bright_yellow(), path.display().to_string().bright_white().bold());
        }
    }

    let pb = create_spinner(format!("{} {} ({})", "Requesting API:".bright_green(), final_source.bright_white().bold(), final_query.bright_cyan()));

    let source = match final_source.as_str() {
        "unsplash" => sources::WallpaperSourceImpl::Unsplash(Unsplash),
        "pexels" => sources::WallpaperSourceImpl::Pexels(Pexels),
        "bing" => sources::WallpaperSourceImpl::Bing(Bing),
        "wallhaven" => sources::WallpaperSourceImpl::Wallhaven(Wallhaven),
        _ => {
            warn!("{} {}", "Unknown source, falling back to wallhaven:".bright_yellow(), final_source.bright_white().bold());
            sources::WallpaperSourceImpl::Wallhaven(Wallhaven)
        }
    };

    let client = reqwest::Client::new();
    match source.fetch(&client, &final_query, &final_color, &final_key, &final_orientation).await {
        Ok(image_url) => {
            pb.finish_and_clear();
            success!("{} {}", "Found Wallpaper:".bright_blue(), image_url.bright_cyan().bold());

            if should_save_wallpaper && save_location.is_some() {
                let loc = save_location.unwrap();
                
                let mut file_name = image_url.split('?').next().unwrap_or(&image_url).split('/').next_back().unwrap_or("wallpaper").to_string();
                
                if image_url.contains("bing.com") && image_url.contains("id=") {
                    if let Some(id_part) = image_url.split("id=").nth(1) {
                        file_name = id_part.split('&').next().unwrap_or(&file_name).to_string();
                    }
                }

                if !["jpg", "png", "jpeg", "webp"].iter().any(|ext| file_name.to_lowercase().ends_with(ext)) {
                    file_name = format!("{}.jpg", file_name);
                }

                let file_path = loc.join(file_name);
                
                let pb_down = create_spinner(format!("{} {}", "Downloading to:".bright_green(), file_path.display().to_string().bright_cyan().bold()));
                
                if let Ok(image_res) = client.get(&image_url).send().await {
                    if let Ok(bytes) = image_res.bytes().await {
                        if std::fs::write(&file_path, bytes).is_ok() {
                            pb_down.finish_and_clear();
                            success!("{} {}", "Saved wallpaper to:".bright_blue(), file_path.display().to_string().bright_cyan().bold());
                            apply_wallpaper(&final_source, &image_url, Some(file_path)).await;
                        } else {
                            pb_down.finish_and_clear();
                            error!("{}", "Failed to save wallpaper".bright_red());
                        }
                    } else {
                        pb_down.finish_and_clear();
                    }
                } else {
                    pb_down.finish_and_clear();
                }
            } else {
                apply_wallpaper(&final_source, &image_url, None).await;
            }
        }
        Err(err) => {
            pb.finish_and_clear();
            error!("{} {}", "Request failed:".bright_red(), err.to_string().bright_white().bold());
        }
    }

    Ok(())
}