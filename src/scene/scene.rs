use crate::scene::light::Light;
use crate::scene::material::Material;
use crate::scene::skybox::Skybox;
use crate::scene::sphere::Sphere;
use crate::scene::triangle::Triangle;
use crate::util::camera::Camera;
use crate::util::hit::Hit;
use crate::util::ray;
use crate::util::vector::Vec3;
use crate::util::{color::Color, ray::Ray};

fn fresnel_schlick(eta1: f32, eta2: f32, cos_a: f32) -> f32 {
    let r0 = ((eta1 - eta2) / (eta1 + eta2)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cos_a).powi(5)
}

pub fn get_pixel(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    spheres: &[Sphere],
    triangles: &[Triangle],
    lights: &[Light],
    camera: &Camera,
    skybox: Option<&Skybox>,
) -> Color {
    let ray = camera.get_ray(x, y, width, height);
    trace_ray(&ray, 15, spheres, triangles, lights, skybox)
}

fn trace_ray(
    ray: &Ray,
    depth: u32,
    spheres: &[Sphere],
    triangles: &[Triangle],
    lights: &[Light],
    skybox: Option<&Skybox>,
) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let nearest_hit = get_hit(spheres, triangles, *ray);
    if nearest_hit.is_none() {
        return match skybox {
            Some(sb) => sb.sample(ray.direction),
            None => Color::new(0.0, 0.0, 0.0),
        };
    }

    let hit = nearest_hit.unwrap();
    let view_dir = (-ray.direction).normalized();
    let offset_shift = hit.normal * 0.0005;

    match hit.material {
        Material::Metal {
            specular_color,
            glossiness,
        } => {
            let reflected_dir = Vec3::reflect(ray.direction, hit.normal);
            let reflected_ray = Ray::new(hit.point + offset_shift, reflected_dir);
            let color = trace_ray(&reflected_ray, depth - 1, spheres, triangles, lights, skybox);
            // NOTE (ARO): Ignore glossiness for now
            specular_color * color
        }
        Material::Dielectric { ior, absorption } => {
            let (eta1, eta2, normal) = if hit.front_face {
                (1.0, ior, hit.normal)
            } else {
                (ior, 1.0, hit.normal)
            };

            let i = ray.direction.normalized();
            let n = hit.normal;

            let reflected_dir = Vec3::reflect(i, n);
            let reflected_ray = Ray::new(hit.point + n * 0.0005, reflected_dir);
            let reflected_color =
                trace_ray(&reflected_ray, depth - 1, spheres, triangles, lights, skybox);

            if let Some(refracted_dir) = Vec3::refract(i, n, eta1, eta2) {
                let refracted_ray = Ray::new(hit.point - n * 0.0005, refracted_dir);
                let refracted_color =
                    trace_ray(&refracted_ray, depth - 1, spheres, triangles, lights, skybox);
                let cos_a = -i.dot(&n);
                let f = fresnel_schlick(eta1, eta2, cos_a);
                (reflected_color * f) + (refracted_color * (1.0 - f))
            } else {
                reflected_color
            }
        }
        Material::BlinnPhong { .. } => {
            let mut color = hit.material.ambient_term();
            for light in lights {
                let to_light = light.position - hit.point;
                let light_dir = to_light.normalized();
                let ray_to_light = ray::Ray::new(hit.point + offset_shift, light_dir);
                let distance_to_light_2 = to_light.length2();
                let mut in_shadow = false;

                for sphere in spheres {
                    if let Some(intersection) = sphere.get_hit(&ray_to_light) {
                        if intersection.lambda > 0.0001
                            && intersection.lambda * intersection.lambda < distance_to_light_2
                        {
                            match intersection.material {
                                Material::Dielectric { .. } => continue,
                                _ => {
                                    in_shadow = true;
                                    break;
                                }
                            }
                        }
                    }
                }

                if !in_shadow {
                    for triangle in triangles {
                        if let Some(intersection) = triangle.get_hit(&ray_to_light) {
                            if intersection.lambda > 0.0001
                                && intersection.lambda * intersection.lambda < distance_to_light_2
                            {
                                match intersection.material {
                                    Material::Dielectric { .. } => continue,
                                    _ => {
                                        in_shadow = true;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }

                if !in_shadow {
                    color += hit
                        .material
                        .shade_blinn_phong(light, hit.normal, light_dir, view_dir);
                }
            }

            color
        }
    }
}

fn get_hit(spheres: &[Sphere], triangles: &[Triangle], ray: Ray) -> Option<Hit> {
    let mut min_lambda = f32::INFINITY;
    let mut nearest_hit: Option<Hit> = None;

    for sphere in spheres {
        if let Some(hit) = sphere.get_hit(&ray) {
            if hit.lambda > 0.00001 && hit.lambda < min_lambda {
                min_lambda = hit.lambda;
                nearest_hit = Some(hit);
            }
        }
    }

    for triangle in triangles {
        if let Some(hit) = triangle.get_hit(&ray) {
            if hit.lambda > 0.00001 && hit.lambda < min_lambda {
                min_lambda = hit.lambda;
                nearest_hit = Some(hit);
            }
        }
    }

    nearest_hit
}
