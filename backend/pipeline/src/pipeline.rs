use dotenvy::dotenv;

use image::{DynamicImage, RgbImage};
use video_rs::{decode::Decoder, Location};

use crate::{models::Frame, utils::dyn_image_from_url};

use crate::models::{FileTypes, InputTask, OutputTask};
use facial_detecting::{config::FacialDetectorModel, predictor::FaceDetector};
use facial_recognizer::{
    config::FacialRecognizerModel, models::RecognizedFaceOutput, predictor::FaceRecognizer,
};
use image_embedding::{config::VisualModel, predictor::ImageVisualize};

pub async fn run_pipeline(task: InputTask) -> OutputTask {
    dotenv().ok();
    let facial_detector_config = FacialDetectorModel::init().unwrap();
    let facial_recognizer_config = FacialRecognizerModel::init().unwrap();
    let image_embedding_config = VisualModel::init().unwrap();

    let detector = FaceDetector::new(
        facial_detector_config.model_path,
        facial_detector_config.model_name,
    );
    let recognizer = FaceRecognizer::new(
        facial_recognizer_config.model_path,
        facial_recognizer_config.model_name,
    );
    let image_visualize = ImageVisualize::new(
        image_embedding_config.model_path,
        image_embedding_config.model_name,
    );

    match task.file_type {
        FileTypes::JPEG => {
            let dyn_img = match task.source {
                Location::File(path) => image::open(path).unwrap(),
                Location::Network(url) => dyn_image_from_url(url).await,
            };
            let image_embedding = image_visualize.predict(&dyn_img);

            let detected_faces = detector.predict(&dyn_img);

            let recognized_faces: Vec<RecognizedFaceOutput> = detected_faces
                .iter()
                .zip(recognizer.predict(&dyn_img, &detected_faces))
                .map(|(face, emb)| RecognizedFaceOutput::from_mergers(face, emb.to_vec()))
                .collect();

            return OutputTask::new(vec![Frame::new(image_embedding, recognized_faces)]);
        }
        FileTypes::MP4 => {
            let mut decoder = Decoder::new(task.source).unwrap();
            let mut frames = vec![];

            for frame in decoder.decode_iter() {
                // Не записывается time_stamp
                if let Ok((_, frame)) = frame {
                    // Возможно чрезмерное извращение и можно просто из курсора писать или что-то подобное
                    let raw_img = frame.as_slice().unwrap();
                    let (height, width, _) = frame.dim();
                    let dyn_img = DynamicImage::from(
                        RgbImage::from_raw(width as _, height as _, raw_img.to_vec()).unwrap(),
                    );

                    let image_embedding = image_visualize.predict(&dyn_img);

                    let detected_faces = detector.predict(&dyn_img);

                    let recognized_faces: Vec<RecognizedFaceOutput> = detected_faces
                        .iter()
                        .zip(recognizer.predict(&dyn_img, &detected_faces))
                        .map(|(face, emb)| RecognizedFaceOutput::from_mergers(face, emb.to_vec()))
                        .collect();

                    frames.push(Frame::new(image_embedding, recognized_faces));
                }
            }
            return OutputTask::new(frames);
        }
        _ => panic!("Now, not supported!"),
    }
}
