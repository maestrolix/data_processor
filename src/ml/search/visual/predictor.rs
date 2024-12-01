use image::{imageops::FilterType, DynamicImage, RgbImage};
use itertools::Itertools;
use ort::{
    inputs,
    session::{builder::GraphOptimizationLevel, Session},
};

#[derive(Debug, Clone)]
pub struct ImageVisualize {
    pub model_path: String,
    pub model_name: String,
}

impl ImageVisualize {
    pub fn new(path: String, name: String) -> Self {
        ImageVisualize {
            model_path: path,
            model_name: name,
        }
    }

    pub fn predict(&self, dyn_image: DynamicImage) -> Vec<f32> {
        let session = self.load_session();

        let tensor = Self::get_tensor(
            &dyn_image
                .resize_to_fill(224, 224, FilterType::CatmullRom)
                .to_rgb8(),
        );

        let outputs = session.run(inputs![tensor].unwrap()).unwrap();
        let tensor = outputs[0].try_extract_tensor().unwrap();
        let embeddings = tensor.view();

        let seq_len = embeddings.shape().get(1).unwrap();

        embeddings
            .iter()
            .copied()
            .chunks(*seq_len)
            .into_iter()
            .map(|b| b.collect())
            .collect::<Vec<Vec<f32>>>()[0]
            .clone()
    }

    fn get_tensor(image: &RgbImage) -> ndarray::Array<f32, ndarray::Dim<[usize; 4]>> {
        let mut pixels =
            ndarray::Array::<f32, ndarray::Dim<[usize; 4]>>::zeros(ndarray::Dim([1, 3, 224, 224]));

        let mean = vec![0.48145466, 0.4578275, 0.40821073];
        let std = vec![0.26862954, 0.261_302_6, 0.27577711];
        for (x, y, pixel) in image.enumerate_pixels() {
            let (x, y) = (x as usize, y as usize);

            pixels[[0, 0, x, y]] = (pixel.0[0] as f32 / 255.0 - mean[0]) / std[0];
            pixels[[0, 1, x, y]] = (pixel.0[1] as f32 / 255.0 - mean[1]) / std[1];
            pixels[[0, 2, x, y]] = (pixel.0[2] as f32 / 255.0 - mean[2]) / std[2];
        }

        pixels
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
