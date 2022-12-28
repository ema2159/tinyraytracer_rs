extern crate image;
extern crate nalgebra;
extern crate piston_window;

mod tinyraytracer;

use std::rc::Rc;

use image::{Rgba, RgbaImage};
use nalgebra::{Point3, Vector3};
use piston_window::EventLoop;

use tinyraytracer::render;
use tinyraytracer::scene_elems::{Camera, Light, Material, Sphere, TraceObj, Plane};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

fn main() {
    let mut img = RgbaImage::from_pixel(WIDTH, HEIGHT, Rgba([0, 0, 0, 255]));

    let camera = Camera {
        fov: 1.0, // Radians
        position: Point3::new(0., 0., 0.),
    };

    // Materials
    let ivory = Rc::new(Material {
        color: Rgba([102, 102, 76, 255]),
        albedo: [0.6, 0.3, 0.1, 0.],
        spec_exponent: 50.,
        refr_ratio: 1.,
    });

    let red_rubber = Rc::new(Material {
        color: Rgba([76, 25, 25, 255]),
        albedo: [0.9, 0.1, 0., 0.],
        spec_exponent: 10.,
        refr_ratio: 1.,
    });

    let mirror = Rc::new(Material {
        color: Rgba([255, 255, 255, 255]),
        albedo: [0.0, 10., 0.8, 0.],
        spec_exponent: 1425.,
        refr_ratio: 1.,
    });

    let glass = Rc::new(Material {
        color: Rgba([255, 255, 255, 255]),
        albedo: [0.0, 0.5, 0.1, 0.8],
        spec_exponent: 125.,
        refr_ratio: 1.5,
    });

    // Objects
    let sphere0 = Sphere {
        center: Point3::new(-3., 0., -16.),
        radius: 2.,
        material: ivory.clone(),
    };
    let sphere1 = Sphere {
        center: Point3::new(-1., -1.5, -12.),
        radius: 2.,
        material: glass.clone(),
    };
    let sphere2 = Sphere {
        center: Point3::new(1.5, -0.5, -18.),
        radius: 3.,
        material: red_rubber.clone(),
    };
    let sphere3 = Sphere {
        center: Point3::new(7., 5., -18.),
        radius: 4.,
        material: mirror.clone(),
    };
    let plane = Plane {
        center: Point3::new(0., -5., 0.),
        normal: Vector3::new(0., 1., 0.),
        material: red_rubber.clone(),
        dims: [3., 4.],
    };

    let objs: Vec<Box<dyn TraceObj>> = vec![
        Box::new(sphere0),
        Box::new(sphere1),
        Box::new(sphere2),
        Box::new(sphere3),
        Box::new(plane),
    ];

    // Light sources
    let light0 = Light {
        position: Point3::new(-20., 20., 20.),
        intensity: 1.5,
    };
    let light1 = Light {
        position: Point3::new(30., 50., -25.),
        intensity: 1.8,
    };
    let light2 = Light {
        position: Point3::new(30., 20., 30.),
        intensity: 1.7,
    };

    let lights: Vec<Light> = vec![light0, light1, light2];

    // Render scene
    use std::time::Instant;
    let now = Instant::now();
    render(&objs, &lights, camera, &mut img);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

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
