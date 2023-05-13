use std::rc::Rc;

use nalgebra::{Point3, Vector3};

use super::{materials::Material, Ray, TraceObj};

#[derive(Debug)]
pub struct Rectangle {
    pub low_left: Point3<f32>,
    pub up_right: Point3<f32>,
    pub material: Rc<dyn Material>,
}

impl Rectangle {
    fn get_width_height_vectors(&self) -> (Vector3<f32>, Vector3<f32>) {
        let width_vec = Vector3::new(
            self.up_right.x - self.low_left.x,
            self.up_right.y - self.low_left.y,
            0.0,
        );
        let height_vec = Vector3::new(0.0, 0.0, self.up_right.z - self.low_left.z);

        (width_vec, height_vec)
    }
}

impl TraceObj for Rectangle {
    fn ray_intersect(&self, ray: &Ray) -> Option<f32> {
        // First, calculate the intersection point (if any) of the ray with the infinite plane that
        // contains the rectangle
        let normal = self.get_normal(Point3::origin());

        let d = -normal.dot(&self.low_left.coords); // Parameter of plane equation

        let n_dot_raydir = -normal.dot(&ray.direction);
        if n_dot_raydir <= 0. {
            return None;
        }

        // If it exists, calculate the intersection point
        let t = (normal.dot(&ray.origin.coords) + d) / n_dot_raydir;
        if t < 0. {
            return None;
        }
        let intersection_point = ray.origin + t * ray.direction;

        // Then, check if the point is inside of the rectangle
        let (width_vec, height_vec) = self.get_width_height_vectors();
        let height = height_vec.norm();
        let width = width_vec.norm();
        let height_dir = height_vec.normalize();
        let width_dir = width_vec.normalize();
        // To do so, project the point into the width and height vectors
        let intersection_vec = intersection_point - self.low_left;
        let height_proj = intersection_vec.dot(&height_dir);
        let width_proj = intersection_vec.dot(&width_dir);
        // Then, verify if such projections fit into the dimensions of the rectangle
        if (0. ..height).contains(&height_proj) && (0. ..width).contains(&width_proj) {
            Some(t)
        } else {
            None
        }
    }

    fn get_normal(&self, _intersect_point: Point3<f32>) -> Vector3<f32> {
        let (width_vec, height_vec) = self.get_width_height_vectors();

        width_vec.cross(&height_vec).normalize()
    }

    fn material(&self) -> &dyn Material {
        &*self.material
    }
}
