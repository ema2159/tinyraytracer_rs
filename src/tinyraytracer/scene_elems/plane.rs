use std::rc::Rc;

use nalgebra::{Point3, Vector3};

use super::{materials::Material, Ray, TraceObj};

#[derive(Debug)]
pub struct Plane {
    pub p0: Point3<f32>,
    pub normal: Vector3<f32>,
    pub material: Rc<dyn Material>,
}

impl TraceObj for Plane {
    fn ray_intersect(&self, ray: &Ray) -> Option<f32> {
        // Calculate using the equation for the intersection between a line and a plane
        let d = -self.normal.dot(&self.p0.coords); // Parameter of plane equation

        let n_dot_raydir = -self.normal.dot(&ray.direction);
        // If 0, ray is parallel to plane. If less than zero, plane is behind ray
        if n_dot_raydir <= 0. {
            return None;
        }

        let t = (self.normal.dot(&ray.origin.coords) + d) / n_dot_raydir;

        Some(t)
    }

    fn get_normal(&self, _intersect_point: Point3<f32>) -> Vector3<f32> {
        self.normal
    }

    fn material(&self) -> &dyn Material {
        &*self.material
    }
}
