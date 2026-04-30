use crate::{scene::material::Material, util::vector::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Hit {
    pub point: Vec3,
    pub normal: Vec3,
    pub lambda: f32,
    pub material: Material,
    pub offset_x: Option<f32>,
    pub offset_y: Option<f32>,
    pub front_face: bool,
}

impl Hit {
    pub fn new(
        point: Vec3,
        normal: Vec3,
        lambda: f32,
        material: Material,
        offset_x: Option<f32>,
        offset_y: Option<f32>,
        ray_direction: Vec3,
    ) -> Self {
        let front_face = ray_direction.dot(&normal) < 0.0;
        let adjusted_normal = if front_face { normal } else { -normal };

        Self {
            point,
            normal: adjusted_normal,
            lambda,
            material,
            offset_x,
            offset_y,
            front_face,
        }
    }
}
