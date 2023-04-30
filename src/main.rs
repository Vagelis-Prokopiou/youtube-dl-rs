use std::io::Write;
use rayon::prelude::*;
use youtube_dl_rs::is_valid_url;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Validate and collect the urls.
    let urls: Vec<String> = args
        .iter()
        .skip(1)
        .map(|v| v.trim().to_owned())
        .filter(|v| is_valid_url(v))
        .collect();

    // Download them.
    urls.par_iter().for_each(|url| {
        println!("Downloading url {url}");
        let output = std::process::Command::new("youtube-dl")
            // Todo: Maybe add it after a related flag.
            //.arg("--verbose")
            .arg("-f")
            .arg("best")
            .arg(url)
            .output();
        match output {
            Ok(v) => {
                std::io::stdout()
                    .write_all(&v.stdout)
                    .expect("Failed to propagate child process stdout");
                std::io::stderr()
                    .write_all(&v.stderr)
                    .expect("Failed to propagate child process stderr");
            }
            Err(e) => {
                std::io::stderr()
                    .write_all(format!("Failed to download url {url} with error {e}").as_bytes())
                    .expect("Failed to write to stderr");
            }
        }
    });
}
