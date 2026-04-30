use crate::{
    scene::material::Material,
    util::{hit::Hit, ray::Ray, vector::Vec3},
};

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn get_hit(&self, ray: &Ray) -> Option<Hit> {
        let v = ray.origin - self.center;
        let u = ray.direction;
        let a = u.dot(&u);
        let b = 2.0 * v.dot(&u);
        let c = v.dot(&v) - self.radius * self.radius;
        let root_content = b * b - 4.0 * a * c;
        if root_content < 0.0 {
            return None;
        }

        let mut lambda = (-b - root_content.sqrt()) / (2.0 * a);

        if lambda <= 0.0001 {
            lambda = (-b + root_content.sqrt()) / (2.0 * a);
        }

        if lambda <= 0.0001 {
            return None;
        }

        let point = ray.origin + ray.direction * lambda;
        let normal = (point - self.center).normalized();
        Some(Hit::new(
            point,
            normal,
            lambda,
            self.material,
            None,
            None,
            ray.direction,
        ))
    }
}
