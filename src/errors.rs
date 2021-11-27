#[derive(Debug)]
pub enum ExtractorError {
    RegexError(regex::Error),
    IoError(std::io::Error),
    HttpError(reqwest::Error),
    Serde(serde_json::Error),
    PlayerResponseNotFound,
    VideoFormatsEmpty,
}

impl From<regex::Error> for ExtractorError {
    fn from(error: regex::Error) -> Self {
        Self::RegexError(error)
    }
}

impl From<std::io::Error> for ExtractorError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<serde_json::Error> for ExtractorError {
    fn from(error: serde_json::Error) -> Self {
        Self::Serde(error)
    }
}

impl From<reqwest::Error> for ExtractorError {
    fn from(error: reqwest::Error) -> Self {
        Self::HttpError(error)
    }
}
