mod errors;
mod yt_extractor;
mod yt_video;

use crate::yt_extractor::YouTubeExtractor;

fn main() {
    match YouTubeExtractor::new("https://youtu.be/7x4-5cakGhk").download() {
        Ok(_) => println!("Download successfully"),
        Err(error) => println!("Error: {:?}", error),
    }
}
