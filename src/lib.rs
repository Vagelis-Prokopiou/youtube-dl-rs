#![allow(clippy::needless_return)]

pub mod models;

pub fn is_valid_url(url: &str) -> bool {
    return url::Url::parse(url).is_ok();
}
