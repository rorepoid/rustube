use std::io::Read;

use serde::Serialize;

use crate::{nc_video::Session, errors::ExtractorError};

#[derive(Debug, Serialize)]
pub struct SessionFactory {
    pub session: SessionData,
}

impl SessionFactory {
    pub fn new(session: &Session) -> Self {
        Self {
            session: SessionData::new(session),
        }
    }

    pub fn get_session(&self, data: String) -> Result<String, ExtractorError> {
        let mut text: String = String::new();
        println!("Extracting video info...");
        let client = reqwest::blocking::Client::new();
        client.post("https://api.dmc.nico/api/sessions?_format=json")
            .body(data)
            .send()?
            .read_to_string(&mut text)?;

        Ok(text)
    }
}

#[derive(Debug, Serialize)]
pub struct SessionData {
    pub recipe_id: String,
    pub content_id: String,
    pub content_type: String,
    pub content_src_id_sets: Vec<ContentSrcIdSet>,
    pub timing_constraint: String,
    pub keep_method: KeepMethod,
    pub protocol: Protocol,
    pub content_uri: String,
    pub session_operation_auth: SessionOperationAuth,
    pub content_auth: ContentAuth,
    pub client_info: ClientInfo,
    pub priority: u32,
}

impl SessionData {
    pub fn new(session: &Session) -> Self {
        Self {
            content_type: String::from("movie"),
            content_src_id_sets: vec![ContentSrcIdSet::new(&session)],
            timing_constraint: String::from("unlimited"),
            keep_method: KeepMethod::new(&session),
            recipe_id: String::from(&session.recipe_id),
            priority: session.priority,
            protocol: Protocol::new(&session),
            content_uri: String::new(),
            session_operation_auth: SessionOperationAuth::new(&session),
            content_id: String::from(&session.content_id),
            content_auth: ContentAuth::new(&session),
            client_info: ClientInfo::new(&session),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ClientInfo {
    pub player_id: String,
}

impl ClientInfo {
    pub fn new(session: &Session) -> Self {
        Self {
            player_id: String::from(&session.player_id),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ContentAuth {
    pub auth_type: String,
    pub content_key_timeout: u32,
    pub service_id: String,
    pub service_user_id: String,
}

impl ContentAuth {
    pub fn new(session: &Session) -> Self {
        Self {
            auth_type: session.auth_types.http.clone(),
            content_key_timeout: session.content_key_timeout,
            service_id: String::from("nicovideo"),
            service_user_id: String::from(&session.service_user_id),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SessionOperationAuth {
    pub session_operation_auth_by_signature: SessionOperationAuthBySignature,
}

impl SessionOperationAuth {
    pub fn new(session: &Session) -> Self {
        Self {
            session_operation_auth_by_signature: SessionOperationAuthBySignature {
                token: String::from(&session.token),
                signature: String::from(&session.signature),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SessionOperationAuthBySignature {
    pub token: String,
    pub signature: String,
}

#[derive(Debug, Serialize)]
pub struct Protocol {
    pub name: String,
    pub parameters: ProtocolParameters,
}

impl Protocol {
    pub fn new(session: &Session) -> Self {
        Self {
            name: String::from("http"),
            parameters: ProtocolParameters::new(&session),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ProtocolParameters {
    pub http_parameters: HttpParameters,
}

impl ProtocolParameters {
    pub fn new(session: &Session) -> Self {
        Self {
            http_parameters: HttpParameters::new(&session),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct HttpParameters {
    pub parameters: Parameters,
}

impl HttpParameters {
    pub fn new(session: &Session) -> Self {
        Self {
            parameters: Parameters::new(&session),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Parameters {
    // pub http_output_download_parameters: HttpOutputDownloadParameters,
    pub hls_parameters: HlsParameters,
}

impl Parameters {
    pub fn new(session: &Session) -> Self {
        Self {
            // http_output_download_parameters: HttpOutputDownloadParameters::new(&session),
            hls_parameters: HlsParameters::new(&session),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct HttpOutputDownloadParameters {
    pub use_well_known_port: String,
    pub use_ssl: String,
    pub transfer_preset: String,
}

impl HttpOutputDownloadParameters {
    pub fn new(session: &Session) -> Self {
        Self {
            use_well_known_port: if session.urls[0].is_well_known_port {
                String::from("yes")
            } else {
                String::from("no")
            },
            use_ssl: String::from("yes"),
            transfer_preset: String::new(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct HlsParameters {
    pub use_well_known_port: String,
    pub use_ssl: String,
    pub transfer_preset: String,
    segment_duration: u32,
}

impl HlsParameters {
    pub fn new(session: &Session) -> Self {
        Self {
            use_well_known_port: if session.urls[0].is_well_known_port {
                String::from("yes")
            } else {
                String::from("no")
            },
            use_ssl: String::from("yes"),
            transfer_preset: String::new(),
            segment_duration: 6000,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct KeepMethod {
    pub heartbeat: Heartbeat,
}

impl KeepMethod {
    pub fn new(session: &Session) -> Self {
        Self {
            heartbeat: Heartbeat {
                lifetime: session.heartbeat_lifetime,
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Heartbeat {
    pub lifetime: u32,
}

#[derive(Debug, Serialize)]
pub struct ContentSrcIdSet {
    pub content_src_ids: Vec<ContentSrcId>,
}

impl ContentSrcIdSet {
    pub fn new(session: &Session) -> Self {
        let audio_src_ids = session.audios.to_owned();
        let video_src_ids = session.videos.to_owned();
        let src_id_to_mux = SrcIdToMux {
            audio_src_ids,
            video_src_ids,
        };
        let src_id_to_mux2 = src_id_to_mux.clone();
        let content_src_id1 = ContentSrcId::new(src_id_to_mux);
        let content_src_id2 = ContentSrcId::new(src_id_to_mux2);

        Self {
            content_src_ids: vec![content_src_id1, content_src_id2],
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ContentSrcId {
    pub src_id_to_mux: SrcIdToMux,
}

impl ContentSrcId {
    pub fn new(src_id_to_mux: SrcIdToMux) -> Self {
        Self { src_id_to_mux }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct SrcIdToMux {
    pub video_src_ids: Vec<String>,
    pub audio_src_ids: Vec<String>,
}
