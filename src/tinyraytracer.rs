pub mod scene_elems;

use std::rc::Rc;

pub use self::scene_elems::materials;
pub use self::scene_elems::{
    Camera, Light, Material, PlainMaterial, Ray, Rectangle, Sphere, TraceObj, Triangle,
};
use image::{Pixel, Rgba, RgbaImage};
use nalgebra::{Point3, Vector3};
use obj::{Obj, Position};

const INTERSECT_LIMIT: f32 = 1000.;
const RAY_DEPTH: u8 = 4;
const ENV_REFR_IDX: f32 = 1.;

/// Check if a given ray intersects any object. Return the nearest intersection distance as well
/// as the nearest object.
fn scene_intersect<'a>(
    ray: &'a Ray,
    objs: &'a Vec<Box<dyn TraceObj>>,
) -> Option<(f32, &'a Box<dyn TraceObj>)> {
    // Placeholders
    let mut intersect_dist = INTERSECT_LIMIT;
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
        Some((intersect_dist, object.unwrap()))
    } else {
        None
    }
}

/// Determine if there is any object between two points. Used to render shadows.
fn single_intersect(
    src_point: Point3<f32>,
    dst_point: Point3<f32>,
    objs: &Vec<Box<dyn TraceObj>>,
) -> bool {
    let ray_dir = -(dst_point - src_point).normalize();
    let ray_dist = (dst_point - src_point).norm() - 1e-3;

    let ray = Ray {
        origin: dst_point,
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

/// Recursively reflect a ray until no intersection is met or until ray depth is reached.
/// Return the resulting reflection color.
fn get_reflection_color(
    ray: &Ray,
    point: Point3<f32>,
    normal: Vector3<f32>,
    objs: &Vec<Box<dyn TraceObj>>,
    lights: &Vec<Light>,
    background: &RgbaImage,
    depth: u8,
) -> Rgba<u8> {
    let ray_dir = reflect_dir(ray.direction, normal);
    // Perturb origin point so ray doesn't intersect with originating object.
    let ray_origin = point
        + normal
            * (if ray_dir.dot(&normal) > 0. {
                1e-3
            } else {
                -1e-3
            });

    let ray = Ray {
        origin: ray_origin,
        direction: ray_dir,
    };
    cast_ray(ray, objs, lights, background, depth + 1)
}

fn refract_dir(
    light_dir: Vector3<f32>,
    normal: Vector3<f32>,
    n1: f32,
    n2: f32,
) -> Option<Vector3<f32>> {
    let cos = -f32::max(-1., f32::min(1., normal.dot(&light_dir)));
    // If ray inside object
    if cos < 0. {
        return refract_dir(light_dir, -normal, n2, n1);
    }

    let eta = n1 / n2;

    let k = 1. - (eta * eta) * (1. - cos);
    if k > 0. {
        let refracted = eta * light_dir + (eta * cos - f32::sqrt(k)) * normal;
        // The ray refracts.
        Some(refracted.normalize())
    } else {
        // Total internal reflection. No refraction occurs.
        None
    }
}

/// Recursively refract a ray until no intersection is met or until ray depth is reached.
/// Return the resulting refr_color color.
fn get_refraction_color(
    ray: &Ray,
    point: Point3<f32>,
    normal: Vector3<f32>,
    refr_ratio: f32,
    objs: &Vec<Box<dyn TraceObj>>,
    lights: &Vec<Light>,
    background: &RgbaImage,
    depth: u8,
) -> Option<Rgba<u8>> {
    if let Some(ray_dir) = refract_dir(ray.direction, normal, ENV_REFR_IDX, refr_ratio) {
        // Perturb origin point so ray doesn't intersect with originating object.
        let ray_origin = point
            + normal
                * (if ray_dir.dot(&normal) > 0. {
                    1e-3
                } else {
                    -1e-3
                });

        let ray = Ray {
            origin: ray_origin,
            direction: ray_dir,
        };
        Some(cast_ray(ray, objs, lights, background, depth + 1))
    } else {
        // Total internal reflection. No refraction
        None
    }
}

/// Get pixel color according to the computed Phong model of the object closest to the camera.
fn get_point_color(
    ray: &Ray,
    point: Point3<f32>,
    normal: Vector3<f32>,
    objs: &Vec<Box<dyn TraceObj>>,
    lights: &Vec<Light>,
    material: &dyn Material,
    background: &RgbaImage,
    depth: u8,
) -> Rgba<u8> {
    let mut diff_light_intensity = 0.;
    let mut spec_light_intensity = 0.;

    for light in lights {
        // Determine if there is any object between the current point and the light source
        if single_intersect(point, light.position, &objs) {
            continue;
        };

        let light_dir = (light.position - point).normalize();
        // Diffuse
        diff_light_intensity += light.intensity * f32::max(0., light_dir.dot(&normal));
        // Specular
        let reflected = reflect_dir(light_dir, normal).dot(&ray.direction);
        spec_light_intensity +=
            f32::powf(f32::max(0., reflected), material.spec_exponent()) * light.intensity;
    }

    // Get reflection image
    let mut reflection = Rgba([0, 0, 0, 0]);
    if material.albedo()[2] > 0. {
        reflection = get_reflection_color(&ray, point, normal, objs, lights, &background, depth);
        reflection.apply_without_alpha(|ch| ((ch as f32) * material.albedo()[2]) as u8);
    }

    // Get refraction image
    let mut refr_color = Rgba([0, 0, 0, 0]);
    if material.albedo()[3] > 0. {
        if let Some(mut refraction) = get_refraction_color(
            &ray,
            point,
            normal,
            material.refr_ratio(),
            objs,
            lights,
            &background,
            depth,
        ) {
            refraction.apply_without_alpha(|ch| ((ch as f32) * material.albedo()[3]) as u8);
            refr_color = refraction;
        }
    }

    // Apply Phong reflection model according to material properties. Also add reflections.
    let mut color_channels = material.color(point).0;
    color_channels[..=2] // Only process R, G, and B channels
        .iter_mut()
        .enumerate()
        .for_each(|(i, ch)| {
            *ch = (*ch as f32 * (diff_light_intensity * material.albedo()[0])
                + 255. * spec_light_intensity * material.albedo()[1]
                + reflection[i] as f32
                + refr_color[i] as f32) as u8;
        });

    Rgba(color_channels)
}

fn get_background(background: &RgbaImage, direction: &Vector3<f32>) -> Rgba<u8> {
    // Calculate spherical coordinates of direction vector
    let (x, y, z) = (direction.x, direction.y, direction.z);

    // Theta: Angle from spherical coordinates that covers half a circle ([-pi, pi]) vertically
    let cos_theta = y; // Given direction is a unit vector, y = cos(theta)

    // Phi: Angle from spherical coordinates that covers a circle ([0, 2*pi]) horizontally
    let cos_phi = f32::cos(z.atan2(x));

    let height_pos = (((cos_theta + 1.) / 2.) * (background.height() - 1) as f32) as u32;
    let width_pos = (((cos_phi + 1.) / 2.) * (background.width() - 1) as f32) as u32;

    *background.get_pixel(width_pos, height_pos)
}

/// Cast a ray. Compute a color according to the elements of the scene the ray intersects.
fn cast_ray(
    ray: Ray,
    objs: &Vec<Box<dyn TraceObj>>,
    lights: &Vec<Light>,
    background: &RgbaImage,
    depth: u8,
) -> Rgba<u8> {
    if depth >= RAY_DEPTH {
        return get_background(&background, &ray.direction);
    }

    if let Some((intersect_dist, object)) = scene_intersect(&ray, &objs) {
        let material = object.material();
        let intersect_point = ray.origin + ray.direction * intersect_dist;
        let normal = object.get_normal(intersect_point);
        let color = get_point_color(
            &ray,
            intersect_point,
            normal,
            objs,
            lights,
            material,
            &background,
            depth,
        );
        color
    } else {
        get_background(&background, &ray.direction)
    }
}

/// Render scene through ray tracing
/// Casts a series of rays that go from an origin (camera position) to each pixel of an image plane.
/// Using such rays, as well as rays casted from the different light sources,the visibilty of each
/// point of each object in the scene is determined,
pub fn render(
    objs: &Vec<Box<dyn TraceObj>>,
    lights: &Vec<Light>,
    camera: &Camera,
    background: &RgbaImage,
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

            let color = cast_ray(
                Ray {
                    origin: camera.position,
                    direction: Vector3::new(i, j, -1.).normalize(),
                },
                objs,
                lights,
                background,
                0,
            );
            img.put_pixel(x, y, color);
        }
    }
}

pub fn push_obj_faces(
    model: &Obj<Position>,
    objs_vec: &mut Vec<Box<dyn TraceObj>>,
    material: Rc<dyn Material>,
) {
    let faces_num = model.indices.len();
    let faces = &model.indices[..faces_num];

    for face in faces.chunks(3) {
        let [v0x, v0y, v0z] = model.vertices[face[0] as usize].position;
        let [v1x, v1y, v1z] = model.vertices[face[1] as usize].position;
        let [v2x, v2y, v2z] = model.vertices[face[2] as usize].position;
        let point0 = Point3::<f32>::new(v0x, v0y, v0z);
        let point1 = Point3::<f32>::new(v1x, v1y, v1z);
        let point2 = Point3::<f32>::new(v2x, v2y, v2z);

        objs_vec.push(Box::new(Triangle {
            a: point0,
            b: point1,
            c: point2,
            material: material.clone(),
        }));
    }
}
