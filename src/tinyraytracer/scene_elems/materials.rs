use image::Rgba;
use nalgebra::Point3;

pub trait Material {
    fn color(&self, intersection_pt: Point3<f32>) -> Rgba<u8>;
    fn albedo(&self) -> [f32; 4];
    fn spec_exponent(&self) -> f32;
    fn refr_ratio(&self) -> f32;
}

pub struct PlainMaterial {
    pub color: Rgba<u8>,
    pub albedo: [f32; 4],
    pub spec_exponent: f32,
    pub refr_ratio: f32,
}

impl Material for PlainMaterial {
    fn color(&self, _intersection_pt: Point3<f32>) -> Rgba<u8> {
        self.color
    }
    fn albedo(&self) -> [f32; 4] {
        self.albedo
    }
    fn spec_exponent(&self) -> f32 {
        self.spec_exponent
    }
    fn refr_ratio(&self) -> f32 {
        self.refr_ratio
    }
}

pub struct CheckerFloorMaterial {
    pub color0: Rgba<u8>,
    pub color1: Rgba<u8>,
    pub albedo: [f32; 4],
    pub spec_exponent: f32,
    pub refr_ratio: f32,
}

impl Material for CheckerFloorMaterial {
    fn color(&self, intersection_pt: Point3<f32>) -> Rgba<u8> {
        if ((0.5 * intersection_pt.x + 1000.) as i32 + (0.5 * intersection_pt.z) as i32) & 1 == 1 {
            self.color0
        } else {
            self.color1
        }
    }
    fn albedo(&self) -> [f32; 4] {
        self.albedo
    }
    fn spec_exponent(&self) -> f32 {
        self.spec_exponent
    }
    fn refr_ratio(&self) -> f32 {
        self.refr_ratio
    }
}
