use image::DynamicImage;

use crate::ml::facial_processing::{FaceDetector, FaceRecognizer};
use crate::models::{DetectedFaceOutput, RecognizedFaceOutput};

pub async fn detecting_faces(
    detector: &FaceDetector,
    image: &DynamicImage,
) -> Vec<DetectedFaceOutput> {
    detector.predict(&image)
}

pub async fn recognition_faces(
    detector: &FaceDetector,
    recognizer: &FaceRecognizer,
    image: &DynamicImage,
) -> Vec<RecognizedFaceOutput> {
    let faces = detector.predict(&image);

    faces
        .iter()
        .zip(recognizer.predict(&image, &faces))
        .map(|(face, emb)| RecognizedFaceOutput::from_mergers(face, emb.to_vec()))
        .collect()
}
