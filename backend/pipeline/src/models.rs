use facial_recognizer::models::RecognizedFaceOutput;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};
use video_rs::{Location, Url};

#[derive(Debug, Deserialize, Serialize)]
pub struct InputMessage {
    link: String,
    link_type: String,
}

pub enum LinkTypes {
    Network,
    Path,
}

#[derive(Debug)]
pub enum FileTypes {
    MP4,
    JPEG,
    PNG,
    SVG,
}

#[derive(Debug)]
pub struct InputTask {
    pub source: Location,
    pub file_type: FileTypes,
}

impl InputTask {
    pub async fn new(key: String, link: String, link_type: String) -> Self {
        let file_type = match key.as_ref() {
            "jpg" => FileTypes::JPEG,
            "jpeg" => FileTypes::JPEG,

            "png" => FileTypes::PNG,
            "svg" => FileTypes::SVG,
            "mp4" => FileTypes::MP4,
            other_type => {
                panic!("Unknown file format - {}", other_type);
            }
        };

        let source = match link_type.as_ref() {
            "file" => Location::File(PathBuf::from_str(&link).unwrap()),
            "network" => Location::Network(link.parse::<Url>().unwrap()),
            other_type => {
                panic!("Unknown link format - {}", other_type);
            }
        };

        InputTask { source, file_type }
    }

    pub async fn from_slice(key: &[u8], data: &[u8]) -> Self {
        let key = String::from_utf8(key.to_vec()).unwrap();
        let message: InputMessage = serde_json::from_slice(data).unwrap();

        InputTask::new(key, message.link, message.link_type).await
    }

    pub async fn from_str(key: &str, data: &str) -> Self {
        let message: InputMessage = serde_json::from_str(data).unwrap();

        InputTask::new(key.to_string(), message.link, message.link_type).await
    }
}

#[derive(Debug, Serialize)]
pub struct Frame {
    pub frame_embedding: Vec<f32>,
    pub faces: Vec<RecognizedFaceOutput>,
}

impl Frame {
    pub fn new(frame_embedding: Vec<f32>, faces: Vec<RecognizedFaceOutput>) -> Self {
        Frame {
            frame_embedding,
            faces,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct OutputTask {
    pub frames: Vec<Frame>,
}

impl OutputTask {
    pub fn new(frames: Vec<Frame>) -> Self {
        OutputTask { frames }
    }
}
