**youtube-dl-rs**: A multithreaded Rust wrapper over youtube-dl, for parallel downloads

- [Description](#description)
- [Build instructions](#build-instructions)
- [Example usage](#example-usage)

# Description

**youtube-dl-rs** is a command line wrapper program over the popular **youtube-dl**. It adds multithreading over
youtube-dl, so that you can parallelize your multiple downloads.

It requires [youtube-dl](https://github.com/ytdl-org/youtube-dl) and [ffmpeg](https://github.com/FFmpeg/FFmpeg).
Check those for installation instructions.

Currently, it is only tested on Linux, since I created it for my own usage, and my main OS is Linux.

# Build instructions

Clone the repository:

```shell
git clone https://github.com/Vagelis-Prokopiou/youtube-dl-rs.git
```

`cd` into it:

```shell
cd youtube-dl-rs
```

Use `cargo` to build:

```shell
cargo build --release
```

Run it:

```shell
./target/release/youtube-dl-rs --help
```

# Example usage

Downloading as `mp4` or next available best option.

```shell
youtube-dl-rs url1 url2 url3 ...
```

Downloading as `mp3` audio files.

```shell
youtube-dl-rs -f mp3 url1 url2 url3 ...
```