use clap::{Parser};
use std::io::Write;
use owo_colors::OwoColorize;

use serde_json::to_string_pretty;
pub mod data;

#[derive(Parser, Debug)]
#[clap(version = "0.1.0", author = "RedBrumbler")]
struct Opts {
    /// The path to the uasset file you want to parse
    file_path: std::path::PathBuf,
    /// optional output path
    #[clap(long="output", short ='o')]
    output_path: Option<std::path::PathBuf>
}

pub fn main() {
    
    let opts = Opts::parse() as Opts;
    let path = opts.file_path;
    let out = opts.output_path.unwrap_or_else(|| path.with_extension("json"));
    if !path.exists() {
        println!("Path {} did not exist, please pass in a valid path", path.to_str().unwrap().bright_yellow());
    }
    let asset = data::u_asset::Asset::read_asset(path);
    let json = to_string_pretty(&asset).unwrap();
    let mut file = std::fs::File::create(&out).unwrap();
    file.write_all(json.as_bytes()).unwrap();

    println!("Dumped asset json to {}", out.to_str().unwrap().bright_yellow());
}