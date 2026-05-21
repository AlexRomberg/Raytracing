mod scene;
mod util;

use wasm_bindgen::prelude::*;

use scene::light::Light;
use scene::material::Material;
use scene::scene::get_pixel;
use scene::sphere::Sphere;

use crate::scene::skybox::Skybox;
use crate::scene::triangle::Triangle;
use crate::util::camera::Camera;
use crate::util::color::Color;
use crate::util::vector::Vec3;

fn parse_spheres(data: &[f32], ambient_intensity: f32) -> Vec<Sphere> {
    data.chunks_exact(11)
        .map(|c| {
            let material_type = c[10] as i32;
            let material = match material_type {
                1 => Material::Metal {
                    specular_color: Color::new(c[4], c[5], c[6]),
                    glossiness: 0.0,
                },
                2 => Material::Dielectric {
                    ior: 1.5,
                    absorption: Color::new(c[4], c[5], c[6]),
                },
                _ => Material::from_color(
                    Color::new(c[4], c[5], c[6]),
                    ambient_intensity,
                    c[7],
                    c[8],
                    c[9],
                ),
            };

            Sphere {
                center: Vec3::new(c[0], c[1], c[2]),
                radius: c[3],
                material,
            }
        })
        .collect()
}

fn parse_triangles(data: &[f32], ambient_intensity: f32) -> Vec<Triangle> {
    data.chunks_exact(16)
        .map(|c| {
            let material_type = c[15] as i32;
            let material = match material_type {
                1 => Material::Metal {
                    specular_color: Color::new(c[9], c[10], c[11]),
                    glossiness: 0.0,
                },
                2 => Material::Dielectric {
                    ior: 1.5,
                    absorption: Color::new(c[9], c[10], c[11]),
                },
                _ => Material::from_color(
                    Color::new(c[9], c[10], c[11]),
                    ambient_intensity,
                    c[12],
                    c[13],
                    c[14],
                ),
            };

            Triangle::new(
                Vec3::new(c[0], c[1], c[2]),
                Vec3::new(c[3], c[4], c[5]),
                Vec3::new(c[6], c[7], c[8]),
                material,
            )
        })
        .collect()
}

fn parse_lights(data: &[f32]) -> Vec<Light> {
    data.chunks_exact(6)
        .map(|c| Light::new(Vec3::new(c[0], c[1], c[2]), Color::new(c[3], c[4], c[5])))
        .collect()
}

#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn render_rows(
    width: u32,
    height: u32,
    start_row: u32,
    end_row: u32,
    sphere_data: &[f32],
    triangle_data: &[f32],
    light_data: &[f32],
    diffuse_intensity: f32,
    skybox_pixels: &[f32],
    skybox_width: u32,
    skybox_height: u32,
    skybox_brightness: f32,
) -> Vec<f32> {
    let spheres = parse_spheres(sphere_data, diffuse_intensity);
    let triangles = parse_triangles(triangle_data, diffuse_intensity);
    let lights = parse_lights(light_data);
    let skybox = Skybox::from_slice(
        skybox_pixels,
        skybox_width,
        skybox_height,
        skybox_brightness,
    );
    let skybox_ref = skybox.as_ref();
    let row_count = end_row - start_row;
    let mut pixels = Vec::with_capacity((row_count * width * 4) as usize);
    let alpha = 1.0f32;
    let camera = Camera::new(
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        90.0,
        width as f32 / height as f32,
    );

    for y in start_row..end_row {
        for x in 0..width {
            let flipped_y = height as f32 - y as f32 - 1.0;
            let color = get_pixel(
                x as f32,
                flipped_y as f32,
                width as f32,
                height as f32,
                &spheres,
                &triangles,
                &lights,
                &camera,
                skybox_ref,
            );
            pixels.push(color.r);
            pixels.push(color.g);
            pixels.push(color.b);
            pixels.push(alpha);
        }
    }

    pixels
}
