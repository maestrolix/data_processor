use image::DynamicImage;

use crate::ml::search::{ImageTextualize, ImageVisualize};

pub async fn clip_textual(textualize: &ImageTextualize, text_query: &str) -> Vec<f32> {
    textualize.predict(text_query)
}

pub async fn clip_visual(visualize: &ImageVisualize, image: &DynamicImage) -> Vec<f32> {
    visualize.predict(image)
}
