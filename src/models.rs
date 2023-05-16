use clap::{Parser};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Format { Default, MP3 }

impl std::str::FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "mp3" => Ok(Format::MP3),
            _ => Ok(Format::Default),
        }
    }
}

impl From<Format> for String {
    fn from(format: Format) -> Self {
        match format {
            Format::Default => String::from("bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best"),
            Format::MP3 => String::from("bestaudio")
        }
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", String::from(*self))
    }
}

/// Rust multithreaded wrapper over youtube-dl
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Video format code, see the "FORMAT SELECTION" for all the info
    #[arg(short, long, default_value_t = Format::Default)]
    pub format: Format,

    // Handled internally
    #[arg(hide = true, short)]
    pub audio_format: Option<String>,

    /// The url(s) to download
    #[arg(required = true, value_name = "URL(s)")]
    pub urls: Vec<String>,
}
