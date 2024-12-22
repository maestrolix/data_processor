use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DetectedFaceOutput {
    pub score: f32,
    pub bbox: [f32; 4],
    pub landmarks: [(f32, f32); 5],
}
