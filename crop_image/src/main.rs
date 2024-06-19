use image::DynamicImage;
mod process;
// use image::io::Reader as ImageReader;

fn main() {
    let input_file = "img000009.bmp";
    let threshold = 60;
    let output_file = format!("output/output-{}.jpg", threshold);

    let img: DynamicImage = image::open(input_file).expect("Failed to open image");
    // let img = ImageReader::open(input_file).expect("Failed to open image").decode().unwrap();

    let img_buffer = process::binarize_image(&img, threshold);

    img_buffer
        .save(output_file)
        .expect("Failed to save binarize_image");
}
