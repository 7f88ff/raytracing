use std::{
    f32::{consts::PI, INFINITY},
    fs::File,
    io::BufWriter,
    path::Path,
    rc::Rc,
};

use rand::Rng;

use raytracing::{
    scale2rgb, Camera, Color, Dielectric, Hittable, Lambertian, Metal, Point3d, Ray, Scene, Sphere,
    Vec3d,
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

fn rand_scene() -> Scene {
    let mut rng = rand::thread_rng();
    let mut s = Scene::new();

    let ground_mtrl = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    s.add(Rc::new(Sphere::new(
        Point3d::new(0., -1000., 0.),
        1000.,
        ground_mtrl,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mtrl = rng.gen::<f32>();
            let c = Point3d::new(
                a as f32 + rng.gen::<f32>(),
                0.2,
                b as f32 + rng.gen::<f32>(),
            );

            if (c - Point3d::new(4., 0.2, 0.)).len() > 0.9 {
                if choose_mtrl < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let mtrl = Rc::new(Lambertian::new(albedo));
                    s.add(Rc::new(Sphere::new(c, 0.2, mtrl)));
                } else if choose_mtrl < 0.95 {
                    let albedo = Color::random() * Color::random();
                    let mtrl = Rc::new(Metal::new(albedo));
                    s.add(Rc::new(Sphere::new(c, 0.2, mtrl)));
                } else {
                    let mtrl = Rc::new(Dielectric::new(1.5));
                    s.add(Rc::new(Sphere::new(c, 0.2, mtrl)));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    s.add(Rc::new(Sphere::new(
        Point3d::new(0., 1., 0.),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    s.add(Rc::new(Sphere::new(
        Point3d::new(-4., 1., 0.),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5)));
    s.add(Rc::new(Sphere::new(
        Point3d::new(4., 1., 0.),
        1.0,
        material3,
    )));

    s
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
    let s = rand_scene();

    // Camera
    let lookfrom = Point3d::new(13., 2., 3.);
    let lookat = Point3d::new(0., 0., 0.);
    let cam = Camera::from(
        lookfrom,
        lookat,
        Vec3d::new(0., 1., 0.),
        PI / 8.,
        ASPECT_RATIO,
        0.1,
        10.,
    );

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
