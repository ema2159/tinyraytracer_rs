use nalgebra::{Point3, Vector3};

pub struct Sphere {
    pub center: Point3<f32>,
    pub radius: f32,
}

pub struct Camera {
    pub img_width: f32,
    pub img_height: f32,
    pub fov: f32,
    pub position: Point3<f32>,
    pub view_dir: Vector3<f32>,
}

pub struct Ray {
    origin: Point3<f32>,
    direction: Vector3<f32>,
}

trait TraceObj {
    fn ray_intersect(&self, ray: Ray) -> bool;
}

impl Sphere {
    fn ray_intersect(&self, ray: Ray) -> Option<f32> {
        // Vector from ray origin to sphere center
        let orig_to_center = self.center - ray.origin;
        // Length of the vector that goes from the ray origin to the vertical line that passes
        // through the sphere's center
        let proj_on_ray = orig_to_center.dot(&ray.direction);
        // Squared distance betw-sqeen sphere center and casted ray
        let sphere_center_to_ray_sq = orig_to_center.norm_squared() - proj_on_ray * proj_on_ray;

        // If line from sphere center to ray is longer than radius, there is no intersection point
        if sphere_center_to_ray_sq <= self.radius * self.radius {
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
}
