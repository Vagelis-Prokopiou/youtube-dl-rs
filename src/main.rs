#![allow(clippy::needless_return)]

use std::io::Write;
use rayon::prelude::*;
use youtube_dl_rs::*;

fn main() {
    ensure_dependencies_are_installed();

    // Keep the ones that failed, for logging.
    let failed_urls = std::sync::Mutex::new(vec![]);

    let args = parse_args();
    let valid_urls = get_valid_urls(&args.urls);

    // Download them.
    valid_urls.par_iter().for_each(|url| {
        println!("Downloading url {url}");
        let mut command = std::process::Command::new("youtube-dl");

        // Removing the ids from the title.
        command.arg("-o").arg("%(title)s.%(ext)s");

        // Providing the format.
        command.arg(url)
            .arg("-f")
            .arg::<String>(args.format.into());

        if let Some(v) = &args.audio_format {
            // You have to provide the -x flag, to use the audio-format.
            command.arg("-x").arg("--audio-format").arg(v.clone());
        }
        let output = command.output();
        match output {
            Ok(v) => {
                let status = v.status;
                if !status.success() {
                    (failed_urls.lock())
                        .expect("Failed to lock the Mutex")
                        .push(url.clone());
                }
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

    // Log the failures
    let failed_urls = failed_urls.lock().unwrap();
    if failed_urls.len() > 0 {
        println!("\nThe following urls reported a non successful exit status:");
        println!("{}", failed_urls.join("\n"));
    }
}