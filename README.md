# ./paperwall - A simple cli-tool for random wallpapers

This cli tool is my first Rust project, it uses various APIs to automatically update desktop wallpaper. It supports multiple sources like [Wallhaven](https://wallhaven.cc/), [Unsplash](https://unsplash.com/), [Pexels](https://pexels.com/) and [Bing](https://bing.com/). It supports all major operating systems (Windows / Mac / Linux). As it's my initial attempt, there may be bugs, and I welcome feedback and contributions to enhance it.

## Installation

You can download binaries for your operating system from the [releases](https://github.com/towsifkafi/paperwall/releases/) section

Or, if you have rust and cargo installed on your system:
```sh
cargo install paperwall
```

### Building from source
if you want to build this tool from source. Run these following commands:
```sh
git clone https://github.com/towsifkafi/paperwall.git
cd paperwall
cargo build --release
# Optionally, move the built executable to a location in your PATH
```

## Usage
This tool is dead simple, to get a random wallpaper you can just run:
```sh
paperwall
```
By default, it'll fetch a pixel art wallpaper from Wallhaven. You can change it by passing `--query` or `-q` argument:
```sh
paperwall -q "pokemon"
```
this will fetch a random wallpaper that has `pokemon` tag. Also, you can search wallpapers via colorcodes:
```sh
paperwall --query "magikarp" --color "ffffff"
```
You can also change the source:
```sh
paperwall --source unsplash --query "nature"
```

### Advanced Usage

You can save your wallpaper history and even revert to previous wallpapers:
```sh
paperwall history list
paperwall history revert
```

Generate shell completions:
```sh
paperwall completion zsh > ~/.zfunc/_paperwall
```

### Configuration

You can configure default options and store your API keys in a configuration file so you don't have to pass them every time. 
By default, `paperwall` looks for a `config.yml` file in your system's config directory:
- **Linux:** `~/.config/paperwall/config.yml`
- **macOS:** `~/Library/Application Support/paperwall/config.yml`
- **Windows:** `C:\Users\Username\AppData\Roaming\paperwall\config.yml`

Alternatively, you can provide a custom config file path using the `--config` flag.

Here's an example `config.yml`:
```yaml
query: "nature"
color: "000000"
source: "unsplash"
orientation: "landscape"
save_wallpaper: true
download_location: "/path/to/wallpapers"
api_keys:
  wallhaven: "your_wallhaven_key"
  unsplash: "your_unsplash_key"
  pexels: "your_pexels_key"
```

### Help Page:
```
Simple program to fetch random image and set it as wallpaper

Usage: paperwall [OPTIONS] [COMMAND]

Commands:
  completion  Generate shell completions
  history     Local history management
  help        Print this message or the help of the given subcommand(s)

Options:
  -q, --query <QUERY>
          Query you want to search at https://wallhaven.cc [default: pixel]
  -c, --color <COLOR>
          Search with colors [Don't add # in hex codes]
      --wallhaven-key <WALLHAVEN_KEY>
          Use API key for Wallhaven
      --unsplash-key <UNSPLASH_KEY>
          Use API key for Unsplash
      --pexels-key <PEXELS_KEY>
          Use API key for Pexels
      --download-location <DOWNLOAD_LOCATION>
          Location to save the wallpaper
      --orientation <ORIENTATION>
          Search for portrait or landscape wallpapers (default: landscape)
      --config <CONFIG>
          Path to a custom config file
  -s, --source <SOURCE>
          Source to fetch wallpaper from (wallhaven, unsplash, pexels, bing)
  -h, --help
          Print help
  -V, --version
          Print version
```
> [!NOTE]
> For API based sources (Wallhaven, Unsplash, Pexels), you may need API keys for specific features or to avoid rate limits. You can pass these via the `--wallhaven-key`, `--unsplash-key`, and `--pexels-key` flags.

## Development
Since this is my first project in rust, there might a lot of bugs. Feel free to submit a Pull Request or post an issue if you encounter any bugs or errors.

## Source

#### Thanks to [WallHaven](https://wallhaven.cc/), [Unsplash](https://unsplash.com/), [Pexels](https://pexels.com/), and [Bing](https://bing.com/) for their cool APIs.

Also, these libraries/links were very helpful resources throughout the development of this project and helped me learn my things related to Rust:
- https://doc.rust-lang.org/
- https://github.com/clap-rs/clap
- https://github.com/johnthagen/min-sized-rust
- https://medium.com/@emilygoldfein/designing-cli-tools-a8e0858f606e
- https://github.com/dirien/rust-cross-compile
