use std::{f32::INFINITY, fs::File, io::BufWriter, path::Path, rc::Rc};

use rand::Rng;

use raytracing::{
    scale2rgb, Camera, Color, Dielectric, Hittable, Lambertian, Metal, Point3d, Ray, Scene, Sphere,
};

fn ray_color(r: &Ray, s: &Scene, depth: i32) -> Color {
    if depth <= 0 {
        Color::new(0., 0., 0.)
    } else if let Some(rec) = s.hit(r, 1e-3, INFINITY) {
        if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
            attenuation * ray_color(&scattered, s, depth - 1)
        } else {
            Color::new(0., 0., 0.)
        }
    } else {
        let unit_dire = r.dire.unit();
        let t = 0.5 * (unit_dire.y() + 1.);
        Color::new(1., 1., 1.) * (1. - t) + Color::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    // random generator
    let mut rng = rand::thread_rng();

    // Image
    const ASPECT_RATIO: f32 = 16. / 9.;
    const IMG_WIDTH: u32 = 1000;
    const IMG_HEIGHT: u32 = (IMG_WIDTH as f32 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: i32 = 50;

    let img_path = Path::new(r"rendered.png");
    let img_file = File::create(img_path).unwrap();
    let ref mut w = BufWriter::new(img_file);
    let mut img_encoder = png::Encoder::new(w, IMG_WIDTH, IMG_HEIGHT);
    img_encoder.set_color(png::ColorType::Rgb);
    let mut img_writer = img_encoder.write_header().unwrap();
    let mut img_data: Vec<u8> = Vec::new();

    // Scene
    let mut s = Scene::new();

    let ground_mat = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let center_mat = Rc::new(Lambertian::new(Color::new(0.5, 0.3, 0.3)));
    let left_mat = Rc::new(Dielectric::new(1.5));
    let right_mat = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));

    s.add(Rc::new(Sphere::new(
        Point3d::new(0., -100.5, -1.),
        100.0,
        ground_mat,
    )));
    s.add(Rc::new(Sphere::new(
        Point3d::new(0., 0., -1.),
        0.5,
        center_mat,
    )));
    s.add(Rc::new(Sphere::new(
        Point3d::new(-1., 0., -1.),
        0.5,
        left_mat,
    )));
    s.add(Rc::new(Sphere::new(
        Point3d::new(1., 0., -1.),
        0.5,
        right_mat,
    )));

    // Camera
    let cam = Camera::default();

    // Render
    let pb = indicatif::ProgressBar::new(IMG_HEIGHT.into());
    for j in (0..IMG_HEIGHT).rev() {
        pb.inc(1);
        let mut cur_pixel = Color::default();
        for i in 0..IMG_WIDTH {
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + rng.gen::<f32>()) / (IMG_WIDTH - 1) as f32;
                let v = (j as f32 + rng.gen::<f32>()) / (IMG_HEIGHT - 1) as f32;
                let r = cam.get_ray(u, v);
                let rendered = ray_color(&r, &s, MAX_DEPTH);
                cur_pixel = cur_pixel + rendered;
            }
            cur_pixel = cur_pixel / SAMPLES_PER_PIXEL as f32;
            img_data.extend(scale2rgb(cur_pixel.sqrt()));
        }
    }

    img_writer.write_image_data(&img_data).unwrap();
    eprintln!("Done!")
}
