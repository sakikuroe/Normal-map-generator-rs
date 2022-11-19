use image::{DynamicImage, GenericImage, GenericImageView};

type RGB = (f64, f64, f64);

fn cross_product(u: RGB, v: RGB) -> RGB {
    (
        u.1 * v.2 - u.2 * v.1,
        u.2 * v.0 - u.0 * v.2,
        u.0 * v.1 - u.1 * v.0,
    )
}

fn normalize(v: RGB) -> RGB {
    let norm = (v.0.powf(2.0) + v.1.powf(2.0) + v.2.powf(2.0)).powf(0.5);
    (v.0 / norm, v.1 / norm, v.2 / norm)
}

/// Args:
///     image: Input image.
///     strength: Strength of gradient, assumed to be about approximately 1.0 to 10.0.
pub fn image_to_normal_map(image: &DynamicImage, strength: f64) -> DynamicImage {
    assert!(image.width() >= 1);
    assert!(image.height() >= 1);

    let gray_image = image.grayscale();

    let mut res = DynamicImage::new_rgb8(image.width(), image.height());
    for i in 0..image.width() {
        for j in 0..image.height() {
            let top = if i > 0 {
                gray_image.get_pixel(i - 1, j)[0]
            } else {
                gray_image.get_pixel(i, j)[0]
            };
            let down = if i < image.width() - 1 {
                gray_image.get_pixel(i + 1, j)[0]
            } else {
                gray_image.get_pixel(i, j)[0]
            };
            let left = if j > 0 {
                gray_image.get_pixel(i, j - 1)[0]
            } else {
                gray_image.get_pixel(i, j)[0]
            };
            let right = if j < image.height() - 1 {
                gray_image.get_pixel(i, j + 1)[0]
            } else {
                gray_image.get_pixel(i, j)[0]
            };
            let u = (0.0, 1.0 / strength * 256.0, right as f64 - left as f64);
            let v = (1.0 / strength * 256.0, 0.0, down as f64 - top as f64);
            let normal = normalize(cross_product(u, v));
            res.put_pixel(
                i,
                j,
                image::Rgba([
                    ((normal.0 + 1.0) / 2.0 * 256.0) as u8,
                    ((normal.1 + 1.0) / 2.0 * 256.0) as u8,
                    (-normal.2 * 256.0) as u8,
                    255,
                ]),
            );
        }
    }

    res
}
