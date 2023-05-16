#![allow(clippy::needless_return)]

use clap::Parser;
use crate::models::{Args, Format};

pub mod models;

pub fn is_valid_url(url: &str) -> bool {
    return url::Url::parse(url).is_ok();
}

pub fn get_valid_urls(urls: &[String]) -> Vec<String> {
    return urls
        .iter()
        .map(|v| v.trim().to_owned())
        .filter(|url| {
            if is_valid_url(url) { return true; }
            println!("Url {url} is not valid. It has been skipped.");
            return false;
        })
        .collect();
}

pub fn ensure_dependencies_are_installed() {
    let necessary_executables = ["youtube-dl", "ffmpeg"];
    for necessary_executable in necessary_executables {
        let result = std::process::Command::new("which").arg(necessary_executable).output();
        match result {
            Ok(output) => {
                let result = String::from_utf8(output.stdout);
                match result {
                    Ok(v) => {
                        if v.is_empty() {
                            panic!("{necessary_executable} is not in the path. Try installing it.");
                        }
                    }
                    Err(e) => { panic!("Checking for {necessary_executable} failed with error {e}"); }
                }
            }
            Err(e) => { panic!("Checking for {necessary_executable} failed with error {e}"); }
        }
    }
}

pub fn parse_args() -> Args {
    let mut args = Args::parse();
    if args.format == Format::MP3 { args.audio_format = Some("mp3".to_string()); }
    return args;
}