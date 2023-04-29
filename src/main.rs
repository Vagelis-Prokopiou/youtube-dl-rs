use rayon::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        panic!("
This program expects a string arguments with all the urls.
Usage example: youtube-dl-rs 'url1 url2 url3'
");
    }

    // Collect them.
    let urls: Vec<String> = args
        .get(1)
        .unwrap()
        .split(' ')
        .collect::<Vec<_>>()
        .iter()
        .map(|v| v.trim().to_owned())
        .filter(|v| !v.is_empty())
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
