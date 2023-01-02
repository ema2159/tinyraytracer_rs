use std::rc::Rc;

use nalgebra::{Point3, Vector3};

use super::{materials::Material, Ray, TraceObj};

pub struct Triangle {
    pub a: Point3<f32>,
    pub b: Point3<f32>,
    pub c: Point3<f32>,
    pub material: Rc<dyn Material>,
}

impl TraceObj for Triangle {
    fn ray_intersect(&self, ray: &Ray) -> Option<f32> {
        // First, calculate the intersection point (if any) of the ray with the infinite plane that
        // contains the triangle
        let normal = self.get_normal(Point3::origin());

        let d = -normal.dot(&self.a.coords); // Parameter of plane equation

        let n_dot_raydir = -normal.dot(&ray.direction);
        if n_dot_raydir <= 0. {
            return None;
        }

        // If it exists, calculate the intersection point
        let t = (normal.dot(&ray.origin.coords) + d) / n_dot_raydir;
        let intersection_point = ray.origin + t * ray.direction;

        // Through barycentric coordinates, calculate if point is inside the triangle
        // edge 0
        let vec_ab = self.b - self.a;
        let vec_ap = intersection_point - self.a;
        let c = vec_ab.cross(&vec_ap);
        let w = normal.dot(&c);
        if w < 0. {
            return None;
        }

        // edge 1
        let vec_bc = self.c - self.b;
        let vec_bp = intersection_point - self.b;
        let c = vec_bc.cross(&vec_bp);
        let u = normal.dot(&c);
        if u < 0. {
            return None;
        }

        // edge 2
        let vec_ca = self.a - self.c;
        let vec_cp = intersection_point - self.c;
        let c = vec_ca.cross(&vec_cp);
        let v = normal.dot(&c);
        if v < 0. {
            return None;
        }
        Some(t)
    }

    fn get_normal(&self, _intersect_point: Point3<f32>) -> Vector3<f32> {
        let vec0 = self.b - self.a;
        let vec1 = self.c - self.a;
        vec0.cross(&vec1).normalize()
    }

    fn material(&self) -> &dyn Material {
        &*self.material
    }
}
