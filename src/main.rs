use rayon::prelude::*;
use std::io::BufRead;

fn main() {
    println!("This program expects input from stdin.");
    println!("Usage example:  echo -e 'url1 \\n url2 \\n url3' | youtube-dl-rs");

    // Collect them.
    let urls: Vec<String> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| match l {
            Ok(v) => {
                return v.trim().to_owned();
            }
            Err(e) => {
                panic!("Failed to read line with error {e}");
            }
        })
        .collect();

    // Download them.
    urls.par_iter().for_each(|url| {
        println!("{}", format!("Downloading url {url}"));
        std::process::Command::new("youtube-dl")
            .arg("-f")
            .arg("best")
            .arg(format!("{url}"))
            .output()
            .expect(&format!("Failed to down load url {url}"));
    });
}
