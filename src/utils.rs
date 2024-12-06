use image::{DynamicImage, EncodableLayout, ImageReader};
use rayon::prelude::*;
use std::io::Cursor;
use video_rs::Url;

pub async fn dyn_image_from_bytes(image_bytes: &[u8]) -> DynamicImage {
    // Данная функция является узким горлышком всей обработки
    ImageReader::new(Cursor::new(image_bytes))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap()
}

pub fn cosine_similarity(vec1: &[f32], vec2: &[f32]) -> f32 {
    let dot_product: f32 = vec1
        .par_iter()
        .zip(vec2.par_iter())
        .map(|(a, b)| a * b)
        .sum();

    let magnitude1: f32 = vec1.par_iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
    let magnitude2: f32 = vec2.par_iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
    dot_product / (magnitude1 * magnitude2)
}

// Вынести в адаптерную логику
pub async fn dyn_image_from_url(url: Url) -> DynamicImage {
    let client = reqwest::Client::new();
    let data = client.get(url).send().await.unwrap().bytes().await.unwrap();

    dyn_image_from_bytes(data.as_bytes()).await
}
