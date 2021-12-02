use regex::Regex;
use std::fs::File;

use crate::errors::ExtractorError;
use crate::yt_video::PlayerResponse;
use reqwest::blocking::Response;
use std::io::{Read, Write};

pub struct YouTubeExtractor {
    url: &'static str,
}
impl YouTubeExtractor {
    pub fn new(url: &'static str) -> Self {
        Self { url }
    }

    pub fn download(self) -> Result<(), ExtractorError> {
        let url = self
            .player_response()?
            .streaming_data
            .best_format()
            .ok_or(ExtractorError::VideoFormatsEmpty)?;

        println!("Downloading video");
        let resp = reqwest::blocking::get(url)?;
        self.create_video(resp)
    }

    fn create_video(self, mut response: Response) -> Result<(), ExtractorError> {
        let mut data = Vec::new();
        let mut file = File::create("video.mp4")?;
        response.read_to_end(&mut data)?;
        file.write_all(&data)?;

        Ok(())
    }

    fn extract(&self) -> Result<String, ExtractorError> {
        let mut text: String = String::new();
        println!("Extracting video info...");
        reqwest::blocking::get(self.url)?.read_to_string(&mut text)?;

        Ok(text)
    }

    fn find_regex(&self, text: &str) -> Option<String> {
        Regex::new(r"(ytInitialPlayerResponse\s*=\s*)(\{.+?});")
            .ok()?
            .captures(text)?
            .get(2)
            .map(|value| value.as_str().to_owned())
    }

    fn player_response(&self) -> Result<PlayerResponse, ExtractorError> {
        let extraction = self.extract()?;

        let player_response = self
            .find_regex(&extraction)
            .ok_or(ExtractorError::PlayerResponseNotFound)?;

        PlayerResponse::new(&player_response)
    }
}
