pub mod scene_elems;

use self::scene_elems::{Camera, Light, Material, Ray, TraceObj};
use image::{Pixel, Rgba, RgbaImage};
use nalgebra::{Point3, Vector3};

const INTERSECT_LIMIT: f32 = 1000.;
const BACKGROUND_COLOR: Rgba<u8> = Rgba([51, 178, 204, 255]);

fn get_point_color(
    ray: Ray,
    point: Point3<f32>,
    normal: Vector3<f32>,
    lights: &Vec<Light>,
    material: &Material,
) -> Rgba<u8> {
    let mut diff_light_intensity = 0.;
    let mut spec_light_intensity = 0.;

    for light in lights {
        let light_dir = (light.position - point).normalize();
        // Diffuse
        diff_light_intensity += light.intensity * f32::max(0., light_dir.dot(&normal));
        // Specular
        let reflected = (light_dir - normal * 2. * normal.dot(&light_dir)).dot(&ray.direction);
        spec_light_intensity +=
            f32::powf(f32::max(0., reflected), material.spec_exponent) * light.intensity;
    }

    // Apply Phong reflection model according to material properties
    let mut color = material.color;
    color.apply_without_alpha(|ch| {
        (ch as f32 * (diff_light_intensity * material.albedo[0])
            + 255. * spec_light_intensity * material.albedo[1]) as u8
    });
    color
}

fn scene_intersect(
    ray: Ray,
    objs: &Vec<Box<dyn TraceObj>>,
    lights: &Vec<Light>,
) -> Option<Rgba<u8>> {
    let mut intersect_dist = f32::INFINITY;
    let mut material = None;
    let mut normal = Vector3::new(0., 0., 0.);
    let mut intersect_point = Point3::<f32>::origin();

    for obj in objs.iter() {
        if let Some(intersection) = obj.ray_intersect(&ray) {
            if intersection < intersect_dist {
                intersect_dist = intersection;
                intersect_point = ray.origin + ray.direction * intersect_dist;
                normal = obj.get_normal(intersect_point);
                material = Some(obj.material());
            }
        }
    }

    if intersect_dist < INTERSECT_LIMIT {
        let material = material.unwrap();
        let color = get_point_color(ray, intersect_point, normal, lights, material);
        Some(color)
    } else {
        None
    }
}

pub fn render(
    objs: &Vec<Box<dyn TraceObj>>,
    lights: &Vec<Light>,
    camera: Camera,
    img: &mut RgbaImage,
) {
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
