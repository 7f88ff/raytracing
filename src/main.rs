use std::{f32::INFINITY, fs::File, io::BufWriter, path::Path, rc::Rc};

use raytracing::{scale2rgb, Color, Hittable, Point3d, Ray, Scene, Sphere, Vec3d};

fn ray_color(r: &Ray, s: &Scene) -> Color {
    if let Some(rec) = s.hit(r, 0., INFINITY) {
        (rec.norm + Color::new(1., 1., 1.)) * 0.5
    } else {
        let unit_dire = r.dire.unit();
        let t = 0.5 * (unit_dire.y() + 1.);
        Color::new(1., 1., 1.) * (1. - t) + Color::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16. / 9.;
    const IMG_WIDTH: u32 = 400;
    const IMG_HEIGHT: u32 = (IMG_WIDTH as f32 / ASPECT_RATIO) as u32;

    let img_path = Path::new(r"rendered.png");
    let img_file = File::create(img_path).unwrap();
    let ref mut w = BufWriter::new(img_file);
    let mut img_encoder = png::Encoder::new(w, IMG_WIDTH, IMG_HEIGHT);
    img_encoder.set_color(png::ColorType::Rgb);
    let mut img_writer = img_encoder.write_header().unwrap();
    let mut img_data: Vec<u8> = Vec::new();

    // Scene
    let mut s = Scene::new();
    s.add(Rc::new(Sphere::new(Point3d::new(0., 0., -1.), 0.5)));
    s.add(Rc::new(Sphere::new(Point3d::new(0., -100.5, -1.), 100.)));

    // Camera
    const VP_HEIGHT: f32 = 2.;
    const VP_WIDTH: f32 = ASPECT_RATIO * VP_HEIGHT;
    const FOCAL_LEN: f32 = 1.;

    let origin = Point3d::new(0., 0., 0.);
    let horiz = Vec3d::new(VP_WIDTH, 0., 0.);
    let verti = Vec3d::new(0., VP_HEIGHT, 0.);
    let base = origin - horiz / 2. - verti / 2. - Vec3d::new(0., 0., FOCAL_LEN);

    // Render
    let pb = indicatif::ProgressBar::new(IMG_HEIGHT.into());
    for j in (0..IMG_HEIGHT).rev() {
        pb.inc(1);
        for i in 0..IMG_WIDTH {
            let u = i as f32 / (IMG_WIDTH - 1) as f32;
            let v = j as f32 / (IMG_HEIGHT - 1) as f32;
            let r = Ray::new(origin, base + horiz * u + verti * v - origin);
            let rendered = ray_color(&r, &s);
            img_data.extend(scale2rgb(rendered));
        }
    }

    img_writer.write_image_data(&img_data).unwrap();
    eprintln!("Done!")
}
