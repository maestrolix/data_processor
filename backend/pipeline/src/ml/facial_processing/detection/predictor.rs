use image::{DynamicImage, Rgba32FImage};
use ndarray::Array;
use ort::{
    inputs,
    session::{builder::GraphOptimizationLevel, Session},
};

use crate::{
    ml::facial_processing::{detection::post_processing::post_processing, transforms::resize},
    models::DetectedFaceOutput,
};

#[derive(Debug, Clone)]
pub struct FaceDetector {
    pub model_path: String,
    pub model_name: String,
}

impl FaceDetector {
    pub fn new(path: String, name: String) -> Self {
        FaceDetector {
            model_path: path,
            model_name: name,
        }
    }

    pub fn predict(&self, image: &DynamicImage) -> Vec<DetectedFaceOutput> {
        let session = self.load_session();

        let resized_image = resize(image, 640, 640);

        let image_tensor = Self::get_tensor(&resized_image.to_rgba32f());

        let outputs = session.run(inputs![image_tensor].unwrap()).unwrap();

        post_processing(outputs, 0.5, image)
    }

    fn get_tensor(image: &Rgba32FImage) -> ndarray::Array<f32, ndarray::Dim<[usize; 4]>> {
        let shape = image.dimensions();

        let input = Array::from_shape_fn(
            (1_usize, 3_usize, shape.0 as usize, shape.1 as usize),
            |(_, c, i, j)| ((image[(j as _, i as _)][c] as f32) - 0.5f32) / 0.5f32,
        );

        return input;
    }

    fn load_session(&self) -> Session {
        Session::builder()
            .unwrap()
            .with_optimization_level(GraphOptimizationLevel::Disable)
            .unwrap()
            .commit_from_file(&self.model_path)
            .unwrap()
    }
}
