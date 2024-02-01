# ./paperwall - A simple cli-tool for random wallpapers

This cli tool is my first Rust project, uses [Wallhaven](https://wallhaven.cc/)'s API to automatically update desktop wallpaper. It supports all major operating systems (Windows / Mac / Linux). As it's my initial attempt, there may be bugs, and I welcome feedback and contributions to enhance it.

## Installation

You can downlaod binaries for your operating system from the [releases](https://github.com/towsifkafi/paperwall/releases/) section

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
By default, it'll fetch a pixel art wallpaper. You can change it by passing `--query` or `-q` argument:
```sh
paperwall -q "pokemon"
```
this will fetch a random wallpaper that has `pokemon` tag. Also, you can search wallpapers via colorcodes:
```sh
paperwall --query "magikarp" --color "ffffff"
```

### Help Page:
```
Simple program to fetch random image and set it as wallpaper

Usage: paperwall [OPTIONS]

Options:
  -q, --query <QUERY>  Query you want to search at https://wallhaven.cc [default: pixel]
  -c, --color <COLOR>  Search with colors [Don't add # in hex codes] [default: ]
  -k, --key <KEY>      Use API key [For searching NSFW images] [default: ]
  -h, --help           Print help
  -V, --version        Print version
```
> [!NOTE]
> Wallhaven does not require an auth key. But if you want to search for NSFW queries, you'll need one. To pass an auth key use the `--key` argument.

## Development
Since this is my first project in rust, there might a lot of bugs. Feel free to submit a Pull Request or post an issue if you encounter any bugs or errors.

## Source

#### Thanks to [WallHaven](https://wallhaven.cc/)'s cool API.

Also, these libraries/links were very helpful resources throughout the development of this project and helped me learn my things related to Rust:
- https://doc.rust-lang.org/
- https://github.com/clap-rs/clap
- https://github.com/johnthagen/min-sized-rust
- https://medium.com/@emilygoldfein/designing-cli-tools-a8e0858f606e
- https://github.com/dirien/rust-cross-compile