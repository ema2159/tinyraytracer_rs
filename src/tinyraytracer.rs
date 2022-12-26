pub mod scene_elems;

use self::scene_elems::{Camera, Light, Material, Ray, TraceObj};
use image::{Pixel, Rgba, RgbaImage};
use nalgebra::{Point3, Vector3};

const INTERSECT_LIMIT: f32 = 1000.;
const BACKGROUND_COLOR: Rgba<u8> = Rgba([51, 178, 204, 255]);

/// Determine if there is any object between two points. Used to render shadows.
fn single_intersect(
    src_point: Point3<f32>,
    dst_point: Point3<f32>,
    point_normal: Vector3<f32>,
    objs: &Vec<Box<dyn TraceObj>>,
) -> bool {
    let ray_dir = (dst_point - src_point).normalize();
    let ray_dist = (dst_point - src_point).norm();
    let ray_origin = src_point
        + point_normal
            * (if ray_dir.dot(&point_normal) > 0. {
                1e-3
            } else {
                -1e-3
            });

    let ray = Ray {
        origin: ray_origin,
        direction: ray_dir,
    };

    for obj in objs.iter() {
        if let Some(intersection) = obj.ray_intersect(&ray) {
            if intersection < ray_dist {
                return true;
            }
        }
    }
    false
}

fn reflect_dir(light_dir: Vector3<f32>, normal: Vector3<f32>) -> Vector3<f32> {
    light_dir - normal * 2. * normal.dot(&light_dir)
}

/// Get pixel color according to the computed Phong model of the object closest to the camera.
fn get_point_color(
    ray: Ray,
    point: Point3<f32>,
    normal: Vector3<f32>,
    objs: &Vec<Box<dyn TraceObj>>,
    lights: &Vec<Light>,
    material: &Material,
) -> Rgba<u8> {
    let mut diff_light_intensity = 0.;
    let mut spec_light_intensity = 0.;

    for light in lights {
        // Determine if there is any object between the current point and the light source
        if single_intersect(point, light.position, normal, &objs) {
            continue;
        };

        let light_dir = (light.position - point).normalize();
        // Diffuse
        diff_light_intensity += light.intensity * f32::max(0., light_dir.dot(&normal));
        // Specular
        let reflected = reflect_dir(light_dir, normal).dot(&ray.direction);
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

/// Compute the interaction of each ray (both from light sources and from the camera) with each
/// object in the scene.
fn scene_intersect(
    ray: Ray,
    objs: &Vec<Box<dyn TraceObj>>,
    lights: &Vec<Light>,
) -> Option<Rgba<u8>> {
    // Placeholders
    let mut intersect_dist = f32::INFINITY;
    let mut object = None;

    for obj in objs.iter() {
        if let Some(intersection) = obj.ray_intersect(&ray) {
            if intersection < intersect_dist {
                intersect_dist = intersection;
                object = Some(obj);
            }
        }
    }

    if intersect_dist < INTERSECT_LIMIT {
        let object = object.unwrap();
        let material = object.material();
        let intersect_point = ray.origin + ray.direction * intersect_dist;
        let normal = object.get_normal(intersect_point);
        let color = get_point_color(ray, intersect_point, normal, objs, lights, material);
        Some(color)
    } else {
        None
    }
}

/// Render scene through ray tracing
/// Casts a series of rays that go from an origin (camera position) to each pixel of an image plane.
/// Using such rays, as well as rays casted from the different light sources,the visibilty of each
/// point of each object in the scene is determined,
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
            // i and j components of the direction of the casted ray
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
