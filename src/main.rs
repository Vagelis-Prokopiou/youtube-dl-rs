use rayon::prelude::*;

fn main() {
    let urls = [
        "https://ok.ru/video/5482599746078",
        "https://ok.ru/video/5504564267550",
        "https://ok.ru/video/5504386599454",
        "https://ok.ru/video/5494500952606",
        "https://ok.ru/video/5483643079198",
        "https://ok.ru/video/5483236755998",
    ];

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
