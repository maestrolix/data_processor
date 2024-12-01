use image::{DynamicImage, RgbImage};
use std::{path::Path, time::Duration};
use tokio::time::sleep;
use video_rs::decode::Decoder;

use crate::ml::facial_processing::{FaceDetector, FaceRecognizer};
use crate::models::{DetectedFaceOutput, RecognizedFaceOutput, Span, VideoFacialRecognitionOutput};
use crate::utils::cosine_similarity;

pub async fn detecting_faces(
    detector: FaceDetector,
    image: &DynamicImage,
) -> Vec<DetectedFaceOutput> {
    let faces = detector.predict(&image);

    faces
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

pub async fn video_facial_recognition(
    detector: &FaceDetector,
    recognizer: &FaceRecognizer,
    video_raw: &[u8],
    search_face: &DynamicImage,
) -> VideoFacialRecognitionOutput {
    loop {
        if let Ok(is_exist) = std::fs::exists("buffer.mp4") {
            if !is_exist {
                std::fs::write("buffer.mp4", video_raw).unwrap();
                let mut decoder = Decoder::new(Path::new("buffer.mp4")).unwrap();

                let search_face_emb =
                    recognizer.predict(&search_face, &detector.predict(&search_face))[0];

                let mut max_cosine_similarity = 0.;
                let mut span_start = 0.;
                let mut span_end = 0.;
                let mut find_face_in_prev = false;

                let mut spans: Vec<Span> = vec![];
                let len_frames = decoder.frames().unwrap() as usize;

                for (num, frame) in decoder.decode_iter().enumerate() {
                    if let Ok((time, frame)) = frame {
                        let raw_img = frame.as_slice().unwrap();
                        let (height, width, _) = frame.dim();
                        let dyn_img = DynamicImage::from(
                            RgbImage::from_raw(width as _, height as _, raw_img.to_vec()).unwrap(),
                        );
                        let faces = recognition_faces(detector, recognizer, &dyn_img).await;

                        let mut face_not_found = true;

                        for face in &faces {
                            let score = cosine_similarity(&face.embedding, &search_face_emb);

                            if score > 0.5 {
                                face_not_found = false;

                                if !find_face_in_prev {
                                    max_cosine_similarity = score;
                                    span_start = time.as_secs();
                                }

                                if max_cosine_similarity < score {
                                    max_cosine_similarity = score
                                }
                                span_end = time.as_secs();
                                println!("Start: {}\nEnd: {}\n", span_start, span_end);
                                find_face_in_prev = true;
                                break;
                            }
                        }

                        let face_not_found_condition =
                            (faces.len() == 0 || face_not_found) && find_face_in_prev;

                        let last_frame_with_face_condition = (len_frames == num) && !face_not_found;

                        if face_not_found_condition || last_frame_with_face_condition {
                            spans.push(Span {
                                start: span_start,
                                end: span_end,
                                max_score: max_cosine_similarity,
                            });
                            find_face_in_prev = false;
                        }
                    } else {
                        break;
                    }
                }
                std::fs::remove_file("buffer.mp4").unwrap();
                return VideoFacialRecognitionOutput { spans };
            }
            println!("We are waiting for the completion of the operation of another request");
            sleep(Duration::from_millis(600)).await;
        }
    }
}
