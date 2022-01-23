use crate::errors::ExtractorError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct PlayerResponse {
    pub media: Media,
}

impl PlayerResponse {
    pub fn new(text: &str) -> Result<Self, ExtractorError> {
        Ok(serde_json::from_str(text)?)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Media {
    pub delivery: Delivery,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Delivery {
    pub movie: Movie,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Movie {
    pub session: Session,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Session {
    pub recipe_id: String,
    pub player_id: String,
    pub videos: Vec<String>,
    pub audios: Vec<String>,
    pub protocols: Vec<String>,
    pub auth_types: AuthType,
    pub service_user_id: String,
    pub token: String,
    pub signature: String,
    pub content_id: String,
    pub heartbeat_lifetime: u32,
    pub content_key_timeout: u32,
    pub priority: u32,
    pub urls: Vec<Url>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct AuthType {
    pub http: String,
    pub hls: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Url {
    pub url: String,
    pub is_well_known_port: bool,
    pub is_ssl: bool,
}
