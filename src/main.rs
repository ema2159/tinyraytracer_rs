extern crate image;
extern crate nalgebra;
extern crate piston_window;

mod tinyraytracer;

use std::rc::Rc;

use image::{Pixel, Rgba, RgbaImage};
use nalgebra::{Point3, Vector3};
use piston_window::EventLoop;

use tinyraytracer::{Camera, Material, Ray, Sphere, TraceObj};

use crate::tinyraytracer::Light;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;
const INTERSECT_LIMIT: f32 = 1000.;
const BACKGROUND_COLOR: Rgba<u8> = Rgba([51, 178, 204, 255]);

fn scene_intersect(
    ray: Ray,
    objs: &Vec<Box<dyn TraceObj>>,
    lights: &Vec<Light>,
) -> Option<Rgba<u8>> {
    let mut intersect_dist = f32::INFINITY;
    let mut color = Rgba([0, 0, 0, 255]);
    let mut normal = Vector3::new(0., 0., 0.);
    let mut intersect_point = Point3::<f32>::origin();
    for obj in objs.iter() {
        if let Some(intersection) = obj.ray_intersect(&ray) {
            if intersection < intersect_dist {
                intersect_dist = intersection;
                intersect_point = ray.origin + ray.direction * intersect_dist;
                normal = obj.get_normal(intersect_point);
                color = obj.material().color;
            }
        }
    }
    if intersect_dist < INTERSECT_LIMIT {
        let diff_light_intensity = calc_intensity(intersect_point, normal, lights);
        color.apply_without_alpha(|ch| ((ch as f32) * diff_light_intensity) as u8);
        Some(color)
    } else {
        None
    }
}

fn calc_intensity(point: Point3<f32>, normal: Vector3<f32>, lights: &Vec<Light>) -> f32 {
    let mut diff_light_intensity = 0.;
    for light in lights {
        let light_dir = (light.position - point).normalize();
        diff_light_intensity += light.intensity * f32::max(0., light_dir.dot(&normal));
    }
    diff_light_intensity
}

fn render(objs: &Vec<Box<dyn TraceObj>>, lights: &Vec<Light>, camera: Camera, img: &mut RgbaImage) {
    let width = img.width() as f32;
    let height = img.height() as f32;
    let y_fov = f32::tan(camera.fov / 2.);
    let x_fov = y_fov * (width / height);
    for x in 0..img.width() {
        for y in 0..img.height() {
            let i = ((2. * (x as f32 + 0.5) / width) - 1.) * x_fov;
            let j = -((2. * (y as f32 + 0.5) / height) - 1.) * y_fov;

            if let Some(color) = scene_intersect(
                Ray {
                    origin: camera.position,
                    direction: Vector3::new(i, j, -1.).normalize(),
                },
                objs,
                lights,
            ) {
                img.put_pixel(x, y, color);
            } else {
                img.put_pixel(x, y, BACKGROUND_COLOR);
            }
        }
    }
}

fn main() {
    let mut img = RgbaImage::from_pixel(WIDTH, HEIGHT, Rgba([0, 0, 0, 255]));

    let camera = Camera {
        fov: 1.0, // Radians
        position: Point3::new(0., 0., 0.),
    };

    let ivory = Rc::new(Material {
        color: Rgba([102, 102, 76, 255]),
    });

    let red_rubber = Rc::new(Material {
        color: Rgba([76, 25, 25, 255]),
    });

    let sphere0 = Sphere {
        center: Point3::new(-3., 0., -16.),
        radius: 2.,
        material: ivory.clone(),
    };
    let sphere1 = Sphere {
        center: Point3::new(-1., -1.5, -12.),
        radius: 2.,
        material: red_rubber.clone(),
    };
    let sphere2 = Sphere {
        center: Point3::new(1.5, -0.5, -18.),
        radius: 3.,
        material: red_rubber.clone(),
    };
    let sphere3 = Sphere {
        center: Point3::new(7., 5., -18.),
        radius: 4.,
        material: ivory.clone(),
    };

    let spheres: Vec<Box<dyn TraceObj>> = vec![
        Box::new(sphere0),
        Box::new(sphere1),
        Box::new(sphere2),
        Box::new(sphere3),
    ];

    let light0 = Light {
        position: Point3::new(-20., 20., 20.),
        intensity: 1.5,
    };
    let lights: Vec<Light> = vec![light0];

    render(&spheres, &lights, camera, &mut img);

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
