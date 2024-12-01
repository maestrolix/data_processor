use serde::{Deserialize, Serialize};

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
}

impl RecognizedFaceOutput {
    pub fn from_mergers(face: &DetectedFaceOutput, embedding: Vec<f32>) -> Self {
        RecognizedFaceOutput {
            score: face.score,
            bbox: face.bbox,
            landmarks: face.landmarks,
            embedding,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Span {
    /// Начало интервала попадания лица (в секундах)
    pub start: f32,
    /// Конец интервала попадания лица (в секундах)
    pub end: f32,
    /// Максимальное совпадение лица в интервале
    pub max_score: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoFacialRecognitionOutput {
    /// Интервалы попадания лица в видео
    pub spans: Vec<Span>,
}
