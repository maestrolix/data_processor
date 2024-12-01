use image::DynamicImage;
use ort::session::SessionOutputs;

use crate::models::DetectedFaceOutput;

const EPS: f32 = 1.0e-7;

pub fn post_processing(
    outputs: SessionOutputs,
    threshold: f32,
    original_image: &DynamicImage,
) -> Vec<DetectedFaceOutput> {
    let mut faces: Vec<DetectedFaceOutput> = vec![];

    let scores08 = &outputs[0].try_extract_tensor().unwrap();
    let scores16 = &outputs[1].try_extract_tensor().unwrap();
    let scores32 = &outputs[2].try_extract_tensor().unwrap();
    let bboxes08 = &outputs[3].try_extract_tensor().unwrap();
    let bboxes16 = &outputs[4].try_extract_tensor().unwrap();
    let bboxes32 = &outputs[5].try_extract_tensor().unwrap();
    let kpsses08 = &outputs[6].try_extract_tensor().unwrap();
    let kpsses16 = &outputs[7].try_extract_tensor().unwrap();
    let kpsses32 = &outputs[8].try_extract_tensor().unwrap();

    for index in 0..12800 {
        let score = scores08[[index, 0]];
        if score > threshold {
            let bbox = distance2bbox(index, 8, bboxes08);
            let landmarks = distance2kps(index, 8, kpsses08);

            faces.push(DetectedFaceOutput {
                score,
                bbox,
                landmarks,
            });
        }
    }

    for index in 0..3200 {
        let score = scores16[[index, 0]];
        if score > threshold {
            let bbox = distance2bbox(index, 16, bboxes16);
            let landmarks = distance2kps(index, 16, kpsses16);

            faces.push(DetectedFaceOutput {
                score,
                bbox,
                landmarks,
            });
        }
    }

    for index in 0..800 {
        let score = scores32[[index, 0]];
        if score > threshold {
            let bbox = distance2bbox(index, 32, bboxes32);
            let landmarks = distance2kps(index, 32, kpsses32);

            faces.push(DetectedFaceOutput {
                score,
                bbox,
                landmarks,
            });
        }
    }

    faces.sort_by(|a, b| (a.score.partial_cmp(&b.score).unwrap()));

    let mut unique_faces = non_maximum_suppression(faces, 0.5);

    normalize_coordinates(&mut unique_faces, original_image);

    unique_faces
}

fn normalize_coordinates(faces: &mut Vec<DetectedFaceOutput>, original_image: &DynamicImage) {
    let (orig_width, orig_height) = (
        original_image.width() as f32,
        original_image.height() as f32,
    );

    let ratio = if orig_width / orig_height > 1. {
        orig_width / 640.
    } else if orig_width / orig_height < 1. {
        orig_height / 640.
    } else {
        1.
    };

    for face in faces {
        face.bbox = face.bbox.map(|el| el * ratio);
        face.landmarks = face.landmarks.map(|(x, y)| (x * ratio, y * ratio));
    }
}

fn distance2bbox(
    index: usize,
    stride: usize,
    distance: &ndarray::prelude::ArrayBase<
        ndarray::ViewRepr<&f32>,
        ndarray::prelude::Dim<ndarray::IxDynImpl>,
    >,
) -> [f32; 4] {
    let m = 640 / stride;
    let x = ((index / 2) * stride) % 640;
    let y = (((index / 2) / m) * stride) % 640;

    let x1 = x as f32 - distance[[index, 0]] * stride as f32;
    let y1 = y as f32 - distance[[index, 1]] * stride as f32;

    let x2 = x as f32 + distance[[index, 2]] * stride as f32;
    let y2 = y as f32 + distance[[index, 3]] * stride as f32;

    return [x1, y1, x2, y2];
}

fn distance2kps(
    index: usize,
    stride: usize,
    distance: &ndarray::prelude::ArrayBase<
        ndarray::ViewRepr<&f32>,
        ndarray::prelude::Dim<ndarray::IxDynImpl>,
    >,
) -> [(f32, f32); 5] {
    let m = 640 / stride;
    let x = ((index / 2) * stride) % 640;
    let y = (((index / 2) / m) * stride) % 640;

    let x1 = x as f32 + distance[[index, 0]] * stride as f32;
    let y1 = y as f32 + distance[[index, 1]] * stride as f32;

    let x2 = x as f32 + distance[[index, 2]] * stride as f32;
    let y2 = y as f32 + distance[[index, 3]] * stride as f32;

    let x3 = x as f32 + distance[[index, 4]] * stride as f32;
    let y3 = y as f32 + distance[[index, 5]] * stride as f32;

    let x4 = x as f32 + distance[[index, 6]] * stride as f32;
    let y4 = y as f32 + distance[[index, 7]] * stride as f32;

    let x5 = x as f32 + distance[[index, 8]] * stride as f32;
    let y5 = y as f32 + distance[[index, 9]] * stride as f32;

    return [(x1, y1), (x2, y2), (x3, y3), (x4, y4), (x5, y5)];
}

/// Запустите не максимальное подавление для возможных ограничивающих рамок.
///
/// Пары ограничивающих рамок с доверием должны быть отсортированы в порядке возрастания
/// достоверности, потому что мы хотим "вытащить()" наиболее достоверные элементы с обратной стороны.
///
/// Начните с наиболее достоверного ограничивающего прямоугольника и выполните итерацию по всем остальным ограничивающим прямоугольникам в
/// порядке уменьшения достоверности. Увеличьте вектор выбранных ограничивающих прямоугольников, добавив только те, которые
/// кандидаты, у которых количество баллов по долговой расписке не превышает "max_iou", с уже выбранными ограничивающими рамками.
/// Выполняется итерация по всем ограничивающим рамкам в `sorted_boxes_with_confidences`. Все кандидаты, набравшие
/// слишком низкие баллы, чтобы их можно было рассматривать, должны быть предварительно отфильтрованы.
fn non_maximum_suppression(
    mut sorted_bboxes_with_confidences: Vec<DetectedFaceOutput>,
    max_iou: f32,
) -> Vec<DetectedFaceOutput> {
    let mut selected: Vec<DetectedFaceOutput> = vec![];
    'candidates: loop {
        match sorted_bboxes_with_confidences.pop() {
            Some(face) => {
                // Проверка на пересечение лиц
                for selected_face in selected.iter() {
                    match iou(&face.bbox, &selected_face.bbox) {
                        x if x > max_iou => continue 'candidates,
                        _ => (),
                    }
                }

                selected.push(face.clone())
            }
            None => break 'candidates,
        }
    }

    selected
}

/// Вычисление показателя пересечения над объединением для двух ограничивающих прямоугольников.
fn iou(bbox_a: &[f32; 4], bbox_b: &[f32; 4]) -> f32 {
    let overlap_box: [f32; 4] = [
        f32::max(bbox_a[0], bbox_b[0]),
        f32::max(bbox_a[1], bbox_b[1]),
        f32::min(bbox_a[2], bbox_b[2]),
        f32::min(bbox_a[3], bbox_b[3]),
    ];

    let overlap_area = bbox_area(&overlap_box);

    overlap_area / (bbox_area(bbox_a) + bbox_area(bbox_b) - overlap_area + EPS)
}

fn bbox_area(bbox: &[f32; 4]) -> f32 {
    let width = bbox[3] - bbox[1];
    let height = bbox[2] - bbox[0];
    if width < 0.0 || height < 0.0 {
        return 0.0;
    }

    width * height
}
