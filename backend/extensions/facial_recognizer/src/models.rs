use facial_detecting::models::DetectedFaceOutput;
use serde::{Deserialize, Serialize};

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
