#![allow(clippy::needless_return)]

pub mod models;

pub fn is_valid_url(url: &str) -> bool {
    return url::Url::parse(url).is_ok();
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