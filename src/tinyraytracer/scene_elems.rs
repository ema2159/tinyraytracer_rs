use std::rc::Rc;

use image::Rgba;
use nalgebra::{Point3, Vector3};

pub struct Sphere {
    pub center: Point3<f32>,
    pub radius: f32,
    pub material: Rc<Material>,
}

pub struct Plane {
    pub p0: Point3<f32>,
    pub normal: Vector3<f32>,
    pub dims: [f32; 2],
    pub material: Rc<Material>,
}

pub struct Rectangle {
    pub low_left: Point3<f32>,
    pub low_right: Point3<f32>,
    pub up_left: Point3<f32>,
    pub material: Rc<Material>,
}

pub struct Light {
    pub position: Point3<f32>,
    pub intensity: f32,
}

pub struct Material {
    pub color: Rgba<u8>,
    pub albedo: [f32; 4],
    pub spec_exponent: f32,
    pub refr_ratio: f32,
}

pub struct Camera {
    pub fov: f32,
    pub position: Point3<f32>,
}

pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
}

pub trait TraceObj {
    fn ray_intersect(&self, ray: &Ray) -> Option<f32>;
    fn get_normal(&self, intersection_point: Point3<f32>) -> Vector3<f32>;
    fn material(&self) -> &Material;
}

impl TraceObj for Sphere {
    fn ray_intersect(&self, ray: &Ray) -> Option<f32> {
        // Vector from ray origin to sphere center
        let orig_to_center = self.center - ray.origin;
        // Length of the vector that goes from the ray origin to the vertical line that passes
        // through the sphere's center
        let proj_on_ray = orig_to_center.dot(&ray.direction);
        // Squared distance betw-sqeen sphere center and casted ray
        let sphere_center_to_ray_sq = orig_to_center.norm_squared() - proj_on_ray * proj_on_ray;

        // If line from sphere center to ray is longer than radius, there is no intersection point
        if sphere_center_to_ray_sq > self.radius * self.radius {
            return None;
        }

        // Distance between the vertical line that passes through the sphere's center and each
        // intersection point
        let centerline_to_intersection =
            f32::sqrt(self.radius * self.radius - sphere_center_to_ray_sq);

        let intersection0 = proj_on_ray - centerline_to_intersection;
        let intersection1 = proj_on_ray + centerline_to_intersection;

        match (intersection0, intersection1) {
            // If first intersection is positive, it is in front of the ray's origin so return that
            _ if intersection0 > 0. => Some(intersection0),
            // If first intersection is negative, it is behind the ray, so if the second one is
            // positive, return that
            _ if intersection1 > 0. => Some(intersection1),
            // If both are negative, both are behind the ray, so there is no intersection
            _ => None,
        }
    }

    fn get_normal(&self, intersect_point: Point3<f32>) -> Vector3<f32> {
        (intersect_point - self.center).normalize()
    }

    fn material(&self) -> &Material {
        &self.material
    }
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

    fn material(&self) -> &Material {
        &self.material
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
        let intersection_point = ray.origin + t * ray.direction;

        // Then, check if the point is inside of the rectangle
        let width_vec = self.low_right - self.low_left;
        let height_vec = self.up_left - self.low_left;
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
        let width_vec = self.low_right - self.low_left;
        let height_vec = self.up_left - self.low_left;

        width_vec.cross(&height_vec).normalize()
    }

    fn material(&self) -> &Material {
        &self.material
    }
}
