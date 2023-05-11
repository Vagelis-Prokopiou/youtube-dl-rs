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
