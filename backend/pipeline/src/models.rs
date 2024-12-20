use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};
use video_rs::{Location, Url};

use crate::ml::{
    facial_processing::{FaceDetector, FaceRecognizer},
    search::{ImageTextualize, ImageVisualize},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DetectedFaceOutput {
    pub score: f32,
    pub bbox: [f32; 4],
    pub landmarks: [(f32, f32); 5],
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecognizedFaceOutput {
    pub score: f32,
    pub bbox: [f32; 4],
    pub landmarks: [(f32, f32); 5],
    pub embedding: Vec<f32>,
    pub time_stamp: Option<f32>,
}

impl RecognizedFaceOutput {
    pub fn from_mergers(face: &DetectedFaceOutput, embedding: Vec<f32>) -> Self {
        RecognizedFaceOutput {
            score: face.score,
            bbox: face.bbox,
            landmarks: face.landmarks,
            embedding,
            time_stamp: None,
        }
    }
}

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

#[derive(Clone)]
pub struct MashineLearning {
    pub detecrot: FaceDetector,
    pub recognizer: FaceRecognizer,
    pub textual: ImageTextualize,
    pub visual: ImageVisualize,
}

impl MashineLearning {
    pub fn from_config(model: &crate::config::Model) -> Self {
        let model = model.clone();
        MashineLearning {
            detecrot: FaceDetector::new(
                model.facial_processing.detector.model_path,
                model.facial_processing.detector.model_name,
            ),
            recognizer: FaceRecognizer::new(
                model.facial_processing.recognizer.model_path,
                model.facial_processing.recognizer.model_name,
            ),
            textual: ImageTextualize::new(
                model.search.textual.model_path,
                model.search.textual.model_name,
            ),
            visual: ImageVisualize::new(
                model.search.visual.model_path,
                model.search.visual.model_name,
            ),
        }
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
