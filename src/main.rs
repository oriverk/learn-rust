use image::{DynamicImage, GenericImageView, ImageBuffer, Luma, Rgba};
// use image::io::Reader as ImageReader;

fn main() {
    let input_file = "img000009.bmp";
    let threshold = 60;
    let output_file = format!("output-{}.jpg", threshold);

    let img: DynamicImage = image::open(input_file).expect("Failed to open image");
    // let img = ImageReader::open(input_file).expect("Failed to open image").decode().unwrap();

    let img_buffer = binarize_image(&img, threshold);

    img_buffer
        .save(output_file)
        .expect("Failed to save binarize_image");
}

/// binarize image
fn binarize_image(img: &DynamicImage, threshold: u16) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut img_buffer = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.pixels() {
        let Rgba([r, g, b, _a]) = pixel;
        let value = calculate_value(r, g, b);
        let binary_value = if value >= (threshold as i16) { 255 } else { 0 };
        img_buffer.put_pixel(x, y, Luma([binary_value]));
    }

    img_buffer
}

/// calculate 2G - R - B
fn calculate_value(r: u8, g: u8, b: u8) -> i16 {
    2 * (g as i16) - (r as i16) - (b as i16)
}
