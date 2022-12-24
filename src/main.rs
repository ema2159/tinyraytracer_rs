extern crate image;
extern crate nalgebra;
extern crate piston_window;

mod tinyraytracer;

use image::{Rgba, RgbaImage};
use nalgebra::{Point3, Vector3};
use piston_window::EventLoop;

use tinyraytracer::{Camera, Sphere};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn main() {
    let mut img = RgbaImage::from_pixel(WIDTH, HEIGHT, Rgba([0, 0, 0, 255]));

    let camera = Camera {
        img_width: 800.,
        img_height: 800.,
        fov: 120.,
        position: Point3::new(0., 0., 5.),
        view_dir: Vector3::new(0., 0., 1.),
    };

    let sphere = Sphere {
        center: Point3::new(0., 0., 0.),
        radius: 4.,
    };

    image::imageops::flip_vertical_in_place(&mut img);

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
