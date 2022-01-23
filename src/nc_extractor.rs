use regex::Regex;

use crate::errors::ExtractorError;
use std::io::Read;

pub struct NicoNicoExtractor {
    url: &'static str,
}

impl NicoNicoExtractor {
    pub fn new(url: &'static str) -> Self {
        Self { url }
    }

    pub fn download(self) -> Result<(), ExtractorError> {
        let player_response = self.player_response()?;
        println!("Downloading video");
        println!("{:?}", player_response);
        Ok(())
    }

    fn find_regex(&self, text: &str) -> Option<String> {
        Regex::new(r#"(data-api-data=")(\{.+?})""#)
            .ok()?
            .captures(text)?
            .get(2)
            .map(|value| value.as_str().replace("&quot;", "\""))
    }

    fn player_response(&self) -> Result<String, ExtractorError> {
        let extraction: String = self.extract()?;

        let player_response: String = self
            .find_regex(&extraction)
            .ok_or(ExtractorError::PlayerResponseNotFound)?;

        Ok(player_response)
    }

    fn extract(&self) -> Result<String, ExtractorError> {
        let mut text: String = String::new();
        println!("Extracting video info...");
        reqwest::blocking::get(self.url)?.read_to_string(&mut text)?;

        Ok(text)
    }
}
