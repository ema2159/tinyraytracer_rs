use image::Rgba;

pub struct PlainMaterial {
    pub color: Rgba<u8>,
    pub albedo: [f32; 4],
    pub spec_exponent: f32,
    pub refr_ratio: f32,
}
