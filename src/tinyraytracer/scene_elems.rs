use nalgebra::{Point3, Vector3};

pub struct Light {
    pub position: Point3<f32>,
    pub intensity: f32,
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
    fn material(&self) -> &dyn Material;
}

// Submodules exports
pub mod materials;
pub mod plane;
pub mod rectangle;
pub mod sphere;
pub use self::materials::*;
pub use self::plane::*;
pub use self::rectangle::*;
pub use self::sphere::*;
