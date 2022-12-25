extern crate image;
extern crate nalgebra;
extern crate piston_window;

mod tinyraytracer;

use image::{Rgba, RgbaImage};
use nalgebra::{Point3, Vector3};
use piston_window::EventLoop;

use tinyraytracer::{Camera, Ray, Sphere, TraceObj};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

fn render(img: &mut RgbaImage, camera: Camera, obj: &dyn TraceObj) {
    let width = img.width() as f32;
    let height = img.height() as f32;
    let y_fov = f32::tan(camera.fov / 2.);
    let x_fov = y_fov * (width / height);
    for x in 0..img.width() {
        for y in 0..img.height() {
            let i = ((2. * (x as f32 + 0.5) / width) - 1.) * x_fov;
            let j = -((2. * (y as f32 + 0.5) / height) - 1.) * y_fov;

            if let Some(_) = obj.ray_intersect(&Ray {
                origin: camera.position,
                direction: Vector3::new(i, j, -1.).normalize(),
            }) {
                img.put_pixel(x, y, Rgba([102, 102, 76, 255]));
            } else {
                img.put_pixel(x, y, Rgba([51, 178, 204, 255]));
            };
        }
    }
}

fn main() {
    let mut img = RgbaImage::from_pixel(WIDTH, HEIGHT, Rgba([0, 0, 0, 255]));

    let camera = Camera {
        fov: 1.0, // Radians
        position: Point3::new(0., 0., 0.),
    };

    let sphere = Sphere {
        center: Point3::new(-3., 0., -16.),
        radius: 2.,
    };

    render(&mut img, camera, &sphere);

    // Rendering window
    let mut window: piston_window::PistonWindow =
        piston_window::WindowSettings::new("tinyraytracer_rs", [WIDTH, HEIGHT])
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|_e| panic!("Could not create window!"));

    window.set_lazy(true);

    let texture = piston_window::Texture::from_image(
        &mut window.create_texture_context(),
        &img,
        &piston_window::TextureSettings::new(),
    )
    .unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g, _| {
            piston_window::clear([0.0, 0.0, 0.0, 1.0], g);
            piston_window::image(&texture, c.transform, g);
        });
    }
}
