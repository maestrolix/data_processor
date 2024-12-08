use image::{DynamicImage, RgbImage};
use video_rs::{decode::Decoder, Location};

use crate::{
    models::Frame,
    services::{facial_processing::recognition_faces, search::clip_visual},
    utils::dyn_image_from_url,
};

use crate::models::{FileTypes, InputTask, MashineLearning, OutputTask};

pub async fn run_pipeline(task: InputTask, ml: &MashineLearning) -> OutputTask {
    match task.file_type {
        FileTypes::JPEG => {
            let dyn_img = match task.source {
                Location::File(path) => image::open(path).unwrap(),
                Location::Network(url) => dyn_image_from_url(url).await,
            };
            let embedding = clip_visual(&ml.visual, &dyn_img).await;
            let faces = recognition_faces(&ml.detecrot, &ml.recognizer, &dyn_img).await;
            return OutputTask::new(vec![Frame::new(embedding, faces)]);
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

                    let embedding = clip_visual(&ml.visual, &dyn_img).await;
                    let faces = recognition_faces(&ml.detecrot, &ml.recognizer, &dyn_img).await;

                    frames.push(Frame::new(embedding, faces));
                }
            }
            return OutputTask::new(frames);
        }
        _ => panic!("Now, not supported!"),
    }
}
