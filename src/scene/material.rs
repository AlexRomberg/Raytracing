use crate::{
    scene::light::Light,
    util::{color::Color, vector::Vec3},
};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    BlinnPhong {
        ambient: Color,
        albedo: Color,
        shininess: f32,
        k_diffuse: f32,
        k_ambient: f32,
        k_specular: f32,
    },
    Metal {
        specular_color: Color,
        glossiness: f32,
    },
    Dielectric {
        ior: f32,
        absorption: Color,
    },
}

impl Material {
    pub fn new(
        ambient: Color,
        albedo: Color,
        shininess: f32,
        k_diffuse: f32,
        k_ambient: f32,
        k_specular: f32,
    ) -> Self {
        Self::BlinnPhong {
            ambient,
            albedo,
            shininess,
            k_diffuse,
            k_ambient,
            k_specular,
        }
    }

    pub fn from_color(
        base_color: Color,
        ambient_intensity: f32,
        shininess: f32,
        k_diffuse: f32,
        k_specular: f32,
    ) -> Self {
        Self::new(
            base_color,
            base_color,
            shininess.max(1.0),
            k_diffuse.max(0.0),
            ambient_intensity.max(0.0),
            k_specular.max(0.0),
        )
    }

    pub fn ambient_term(&self) -> Color {
        match self {
            Self::BlinnPhong {
                ambient, k_ambient, ..
            } => *ambient * *k_ambient,
            _ => Color::new(0.0, 0.0, 0.0),
        }
    }

    pub fn shade_blinn_phong(
        &self,
        light: &Light,
        normal: Vec3,
        light_dir: Vec3,
        view_dir: Vec3,
    ) -> Color {
        match self {
            Self::BlinnPhong {
                albedo,
                shininess,
                k_diffuse,
                k_specular,
                ..
            } => {
                let n = normal.normalized();
                let l = light_dir.normalized();
                let v = view_dir.normalized();

                let ndotl = n.dot(&l).max(0.0);
                if ndotl <= 0.0 {
                    return Color::new(0.0, 0.0, 0.0);
                }

                let diffuse = *albedo * light.color * (*k_diffuse * ndotl);

                let half_vec = l + v;
                let specular_factor = if half_vec.length2() > 0.0 {
                    n.dot(&half_vec.normalized()).max(0.0).powf(*shininess)
                } else {
                    0.0
                };

                let specular = light.color * (*k_specular * specular_factor);

                diffuse + specular
            }
            _ => Color::new(0.0, 0.0, 0.0),
        }
    }
}
