use image::{DynamicImage, Rgba32FImage};

/// Изменяет размер изображения с сохранением соотношения сторон,
/// компенсирует широту или высоту изображения черными пикселями.
/// На выходе получаем изображение `width`*`height`
pub fn resize(image: &DynamicImage, width: u32, height: u32) -> DynamicImage {
    let input = image
        .resize(width, height, image::imageops::FilterType::Nearest)
        .to_rgba32f();

    let mut output = Rgba32FImage::new(width, height);
    for out_row in 0..width {
        for out_col in 0..height {
            match input.get_pixel_checked(out_row as _, out_col as _) {
                Some(pixel) => output.put_pixel(out_row, out_col, *pixel),
                None => output.put_pixel(out_row, out_col, image::Rgba([0., 0., 0., 1.])),
            }
        }
    }

    DynamicImage::from(output)
}
