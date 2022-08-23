use std::{fs::File, io::BufWriter, path::Path};

use raytracing::{scale2rgb, Color};

fn main() {
    // Image
    const IMG_WIDTH: u32 = 256;
    const IMG_HEIGHT: u32 = 256;

    let img_path = Path::new(r"rendered.png");
    let img_file = File::create(img_path).unwrap();
    let ref mut w = BufWriter::new(img_file);
    let mut img_encoder = png::Encoder::new(w, IMG_WIDTH, IMG_HEIGHT);
    img_encoder.set_color(png::ColorType::Rgb);
    let mut img_writer = img_encoder.write_header().unwrap();
    let mut img_data: Vec<u8> = Vec::new();

    // Render
    let pb = indicatif::ProgressBar::new(IMG_HEIGHT.into());
    for j in (0..IMG_HEIGHT).rev() {
        pb.inc(1);
        for i in 0..IMG_WIDTH {
            let c = Color::new(
                i as f32 / (IMG_WIDTH - 1) as f32,
                j as f32 / (IMG_HEIGHT - 1) as f32,
                0.25,
            );
            img_data.extend(scale2rgb(c));
        }
    }

    img_writer.write_image_data(&img_data).unwrap();
    eprintln!("Done!")
}
