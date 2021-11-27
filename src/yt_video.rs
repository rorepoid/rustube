use crate::errors::ExtractorError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct PlayerResponse {
    pub streaming_data: StreamingData,
}

impl PlayerResponse {
    pub fn new(text: &str) -> Result<Self, ExtractorError> {
        Ok(serde_json::from_str(text)?)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct StreamingData {
    pub formats: Vec<VideoFormat>,
}

impl StreamingData {
    pub fn best_format(self) -> Option<String> {
        let all_formats = self.formats;
        if all_formats.is_empty() {
            return None;
        }

        let best = all_formats
            .iter()
            .reduce(|a, b| if a.height >= b.height { a } else { b });

        best.map(|video| video.url.to_owned())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct VideoFormat {
    itag: u8,
    url: String,
    mime_type: String,
    width: u32,
    height: u32,
    quality: String,
    quality_label: String,
    audio_quality: String,
    approx_duration_ms: String,
    audio_sample_rate: String,
}
