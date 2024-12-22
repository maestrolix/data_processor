use dotenvy::dotenv;
use facial_detecting::{config::FacialDetectorModel, predictor::FaceDetector};
use facial_recognizer::{
    config::FacialRecognizerModel, models::RecognizedFaceOutput, predictor::FaceRecognizer,
};
use image::{DynamicImage, RgbImage};
use image_embedding::{config::VisualModel, predictor::ImageVisualize};
use video_rs::{decode::Decoder, Url};

#[tokio::main]
async fn main() {
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

    let source_net =
        "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4"
            .parse::<Url>()
            .unwrap();

    let mut decoder = Decoder::new(source_net).unwrap();
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

            frames.push((recognized_faces, image_embedding));
        }
    }
}
