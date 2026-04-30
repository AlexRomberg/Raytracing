use crate::{
    scene::material::Material,
    util::{hit::Hit, matrices::Matrix3x3, ray::Ray, vector::Vec3},
};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub point_a: Vec3,
    pub point_b: Vec3,
    pub point_c: Vec3,

    pub w: Vec3,
    pub v: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Triangle {
    pub fn new(point_a: Vec3, point_b: Vec3, point_c: Vec3, material: Material) -> Self {
        let w = point_b - point_a;
        let v = point_c - point_a;
        let normal = w.cross(&v).normalized();
        Self {
            point_a,
            point_b,
            point_c,
            w,
            v,
            normal,
            material,
        }
    }

    pub fn get_hit(&self, ray: &Ray) -> Option<Hit> {
        let a = Matrix3x3::from_vectors([ray.direction, -self.v, -self.w]);
        let b = self.point_a - ray.origin;

        if a.determinant() == 0.0 {
            return None;
        }

        let hit_offsets = a.inverse() * b;
        let lambda = hit_offsets.x;
        let mu = hit_offsets.y;
        let tao = hit_offsets.z;

        if lambda <= 0.0001 || mu < 0.0 || tao < 0.0 || mu + tao > 1.0 {
            return None;
        }

        Some(Hit::new(
            ray.origin + (ray.direction * hit_offsets.x),
            self.normal,
            hit_offsets.x,
            self.material,
            Some(hit_offsets.y),
            Some(hit_offsets.z),
            ray.direction,
        ))
    }
}
