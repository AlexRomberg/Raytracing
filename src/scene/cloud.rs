use crate::util::color::Color;
use crate::util::noise::fbm_3d;
use crate::util::ray::Ray;
use crate::util::vector::Vec3;

pub struct Cloud {
    pub center: Vec3,
    pub size: Vec3,
    pub density: f32,
    pub noise_scale: f32,
    pub octaves: u32,
    pub seed: u32,
    pub color: Color,
}

impl Cloud {
    pub fn aabb_intersect(&self, ray: &Ray) -> Option<(f32, f32)> {
        let half = self.size * 0.5;
        let min = self.center - half;
        let max = self.center + half;

        let inv_dx = 1.0 / ray.direction.x;
        let inv_dy = 1.0 / ray.direction.y;
        let inv_dz = 1.0 / ray.direction.z;

        let tx1 = (min.x - ray.origin.x) * inv_dx;
        let tx2 = (max.x - ray.origin.x) * inv_dx;
        let ty1 = (min.y - ray.origin.y) * inv_dy;
        let ty2 = (max.y - ray.origin.y) * inv_dy;
        let tz1 = (min.z - ray.origin.z) * inv_dz;
        let tz2 = (max.z - ray.origin.z) * inv_dz;

        let tmin = tx1.min(tx2).max(ty1.min(ty2)).max(tz1.min(tz2));
        let tmax = tx1.max(tx2).min(ty1.max(ty2)).min(tz1.max(tz2));

        if tmax < 0.0 || tmin > tmax {
            None
        } else {
            Some((tmin.max(0.0), tmax))
        }
    }

    fn sample_density(&self, point: Vec3) -> f32 {
        let local = point - self.center;
        let half = Vec3::new(
            self.size.x.max(1e-6) * 0.5,
            self.size.y.max(1e-6) * 0.5,
            self.size.z.max(1e-6) * 0.5,
        );
        let nx = (local.x / half.x).abs().min(1.0);
        let ny = (local.y / half.y).abs().min(1.0);
        let nz = (local.z / half.z).abs().min(1.0);
        let falloff = (1.0 - nx) * (1.0 - ny) * (1.0 - nz);
        if falloff <= 0.0 {
            return 0.0;
        }

        let s = self.noise_scale;
        let n = fbm_3d(
            point.x * s,
            point.y * s,
            point.z * s,
            self.octaves.max(1),
            0.5,
            2.0,
            self.seed,
        );

        let n_remapped = (n * 0.5 + 0.5 - 0.35).max(0.0);

        const SIGMA_SCALE: f32 = 0.002;
        n_remapped * falloff * self.density * SIGMA_SCALE
    }
}

const MARCH_STEPS: u32 = 50;
const MIN_TRANSMITTANCE: f32 = 0.01;

pub fn march_clouds(ray: &Ray, max_t: f32, clouds: &[Cloud]) -> (Color, f32) {
    let mut color = Color::new(0.0, 0.0, 0.0);
    let mut transmittance = 1.0;

    for cloud in clouds {
        if transmittance < MIN_TRANSMITTANCE {
            break;
        }
        let Some((t_enter, t_exit)) = cloud.aabb_intersect(ray) else {
            continue;
        };
        let t_start = t_enter.max(0.0);
        let t_end = t_exit.min(max_t);
        if t_end <= t_start {
            continue;
        }

        let step_size = (t_end - t_start) / MARCH_STEPS as f32;
        if step_size <= 0.0 {
            continue;
        }

        let mut t = t_start + step_size * 0.5;
        for _ in 0..MARCH_STEPS {
            if transmittance < MIN_TRANSMITTANCE {
                break;
            }
            let point = ray.origin + ray.direction * t;
            let sigma = cloud.sample_density(point);
            if sigma > 0.0 {
                let local_t = (-sigma * step_size).exp();
                let alpha = 1.0 - local_t;
                color += cloud.color * (alpha * transmittance);
                transmittance *= local_t;
            }
            t += step_size;
        }
    }

    (color, transmittance)
}
