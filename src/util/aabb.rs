use crate::util::ray::Ray;
use crate::util::vector::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn empty() -> Self {
        Self {
            min: Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
            max: Vec3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
        }
    }

    pub fn from_triangle(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self {
            min: Vec3::new(
                a.x.min(b.x).min(c.x),
                a.y.min(b.y).min(c.y),
                a.z.min(b.z).min(c.z),
            ),
            max: Vec3::new(
                a.x.max(b.x).max(c.x),
                a.y.max(b.y).max(c.y),
                a.z.max(b.z).max(c.z),
            ),
        }
    }

    pub fn union(a: Aabb, b: Aabb) -> Self {
        Self {
            min: Vec3::new(
                a.min.x.min(b.min.x),
                a.min.y.min(b.min.y),
                a.min.z.min(b.min.z),
            ),
            max: Vec3::new(
                a.max.x.max(b.max.x),
                a.max.y.max(b.max.y),
                a.max.z.max(b.max.z),
            ),
        }
    }

    pub fn extend_point(&mut self, p: Vec3) {
        self.min.x = self.min.x.min(p.x);
        self.min.y = self.min.y.min(p.y);
        self.min.z = self.min.z.min(p.z);
        self.max.x = self.max.x.max(p.x);
        self.max.y = self.max.y.max(p.y);
        self.max.z = self.max.z.max(p.z);
    }

    pub fn longest_axis(&self) -> usize {
        let dx = self.max.x - self.min.x;
        let dy = self.max.y - self.min.y;
        let dz = self.max.z - self.min.z;
        if dx >= dy && dx >= dz {
            0
        } else if dy >= dz {
            1
        } else {
            2
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<(f32, f32)> {
        let inv_dx = 1.0 / ray.direction.x;
        let inv_dy = 1.0 / ray.direction.y;
        let inv_dz = 1.0 / ray.direction.z;

        let tx1 = (self.min.x - ray.origin.x) * inv_dx;
        let tx2 = (self.max.x - ray.origin.x) * inv_dx;
        let ty1 = (self.min.y - ray.origin.y) * inv_dy;
        let ty2 = (self.max.y - ray.origin.y) * inv_dy;
        let tz1 = (self.min.z - ray.origin.z) * inv_dz;
        let tz2 = (self.max.z - ray.origin.z) * inv_dz;

        let tmin = tx1.min(tx2).max(ty1.min(ty2)).max(tz1.min(tz2));
        let tmax = tx1.max(tx2).min(ty1.max(ty2)).min(tz1.max(tz2));

        if tmax < 0.0 || tmin > tmax {
            None
        } else {
            Some((tmin, tmax))
        }
    }
}
