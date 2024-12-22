use std::ops::Mul;

use image::{DynamicImage, Rgba32FImage};
use nalgebra::Matrix3;
use nalgebra::{ArrayStorage, Matrix1x2, Matrix2, Matrix2x1, Matrix3x1};

pub fn warp_into(input: &Rgba32FImage, matrix: Matrix3<f32>, output: &mut Rgba32FImage) {
    let inverse = matrix.try_inverse().unwrap();

    let in_width = input.width();
    let in_height = input.height();

    let out_width = output.width();
    let out_height = output.height();

    for out_row in 0..out_width {
        for out_col in 0..out_height {
            let point = Matrix3x1::<f32>::new(out_row as f32, out_col as f32, 1f32);

            let in_pixel = inverse * point;

            let in_row = in_pixel.x as i32;
            let in_col = in_pixel.y as i32;

            if (0 <= in_row)
                && (in_row < in_width as i32)
                && (0 <= in_col)
                && (in_col < in_height as i32)
            {
                let px = input.get_pixel(in_row as _, in_col as _);

                output[(out_row, out_col)] = *px;
            }
        }
    }
}

/// Алгоритм `Кабша-Умеямы` - это метод нахождения оптимального перемещения, поворота
/// и масштабирования, который выравнивает два набора точек с минимальным среднеквадратичным отклонением (RMSD).
pub fn umeyama<const R: usize>(src: &[(f32, f32); R], dst: &[(f32, f32); R]) -> Matrix3<f32> {
    let src_x_sum: f32 = src.iter().map(|v| v.0).sum();
    let src_x_mean = src_x_sum / (R as f32);

    let src_y_sum: f32 = src.iter().map(|v| v.1).sum();
    let src_y_mean = src_y_sum / (R as f32);

    let dst_x_sum: f32 = dst.iter().map(|v| v.0).sum();
    let dst_x_mean = dst_x_sum / (R as f32);

    let dst_y_sum: f32 = dst.iter().map(|v| v.1).sum();
    let dst_y_mean = dst_y_sum / (R as f32);

    let src_demean_s = ArrayStorage(src.map(|v| [v.0 - src_x_mean, v.1 - src_y_mean]));
    let dst_demean_s = ArrayStorage(dst.map(|v| [v.0 - dst_x_mean, v.1 - dst_y_mean]));

    let src_demean = nalgebra::Matrix::from_array_storage(src_demean_s);
    let dst_demean = nalgebra::Matrix::from_array_storage(dst_demean_s);

    let a = std::ops::Mul::mul(dst_demean, &src_demean.transpose()) / (R as f32);
    let svd = nalgebra::Matrix::svd(a, true, true);

    let determinant = a.determinant();

    let mut d = [1f32; 2];

    if determinant < 0.0f32 {
        d[2 - 1] = -1.0f32;
    }

    let mut t = Matrix2::<f32>::identity();
    let s = svd.singular_values;
    let u = svd.u.unwrap();
    let v = svd.v_t.unwrap();

    let rank = a.rank(0.00001f32);

    if rank == 0 {
        panic!("Matrix rank is 0.");
    } else if rank == 2 - 1 {
        if u.determinant() * v.determinant() > 0.0 {
            u.mul_to(&v, &mut t);
        } else {
            let s = d[2 - 1];
            d[2 - 1] = -1f32;
            let dg = Matrix2::<f32>::new(d[0], 0f32, 0f32, d[1]);

            let udg = u.mul(&dg);
            udg.mul_to(&v, &mut t);
            d[2 - 1] = s;
        }
    } else {
        let dg = Matrix2::<f32>::new(d[0], 0f32, 0f32, d[1]);
        let udg = u.mul(&dg);
        udg.mul_to(&v, &mut t);
    }

    let ddd = Matrix1x2::new(d[0], d[1]);
    let d_x_s = ddd.mul(s);

    let var0 = src_demean.remove_row(0).variance();
    let var1 = src_demean.remove_row(1).variance();

    let varsum = var0 + var1;

    let scale = d_x_s.get((0, 0)).unwrap() / varsum;

    let dst_mean = Matrix2x1::<f32>::new(dst_x_mean, dst_y_mean);
    let src_mean = Matrix2x1::<f32>::new(src_x_mean, src_y_mean);
    let t_x_srcmean = t.mul(&src_mean);

    let xxx = scale * t_x_srcmean;
    let yyy = dst_mean - xxx;

    let m13 = *yyy.get(0).unwrap();
    let m23 = *yyy.get(1).unwrap();

    let m00x22 = t * scale;

    let m11 = m00x22.m11;
    let m21 = m00x22.m21;
    let m12 = m00x22.m12;
    let m22 = m00x22.m22;

    let tt = Matrix3::<f32>::new(m11, m12, m13, m21, m22, m23, 0f32, 0f32, 1f32);

    return tt;
}

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

pub fn crop_face(image: &Rgba32FImage, landmarks: &[(f32, f32); 5], size: u32) -> Rgba32FImage {
    let m = umeyama(&landmarks, &ARCFACE_DST);

    let mut output = Rgba32FImage::new(size, size);
    warp_into(image, m, &mut output);
    output
}

const ARCFACE_DST: [(f32, f32); 5] = [
    (38.2946, 51.6963),
    (73.5318, 51.5014),
    (56.0252, 71.7366),
    (41.5493, 92.3655),
    (70.7299, 92.2041),
];
