mod scene;
mod util;

use wasm_bindgen::prelude::*;

use scene::light::Light;
use scene::material::Material;
use scene::scene::get_pixel;
use scene::sphere::Sphere;

use crate::scene::bvh::Bvh;
use crate::scene::cloud::Cloud;
use crate::scene::skybox::Skybox;
use crate::scene::terrain::{generate as generate_terrain_mesh, TerrainConfig};
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

fn parse_clouds(data: &[f32]) -> Vec<Cloud> {
    data.chunks_exact(13)
        .map(|c| Cloud {
            center: Vec3::new(c[0], c[1], c[2]),
            size: Vec3::new(c[3], c[4], c[5]),
            density: c[6],
            noise_scale: c[7],
            octaves: c[8] as u32,
            seed: c[9] as u32,
            color: Color::new(c[10], c[11], c[12]),
        })
        .collect()
}

fn parse_lights(data: &[f32]) -> Vec<Light> {
    data.chunks_exact(6)
        .map(|c| Light::new(Vec3::new(c[0], c[1], c[2]), Color::new(c[3], c[4], c[5])))
        .collect()
}

#[wasm_bindgen]
pub fn generate_terrain(
    grid_size: u32,
    width: f32,
    depth: f32,
    height_scale: f32,
    octaves: u32,
    persistence: f32,
    lacunarity: f32,
    seed: u32,
) -> Vec<f32> {
    let mesh = generate_terrain_mesh(&TerrainConfig {
        grid_size,
        width,
        depth,
        height_scale,
        octaves,
        persistence,
        lacunarity,
        seed,
    });

    let vc = mesh.vertices.len();
    let fc = mesh.faces.len();
    let mut buf = Vec::with_capacity(2 + 3 * vc + 6 * fc);
    buf.push(vc as f32);
    buf.push(fc as f32);
    for v in &mesh.vertices {
        buf.push(v.x);
        buf.push(v.y);
        buf.push(v.z);
    }
    for f in &mesh.faces {
        buf.push(f[0] as f32);
        buf.push(f[1] as f32);
        buf.push(f[2] as f32);
    }
    for c in &mesh.face_colors {
        buf.push(c.r);
        buf.push(c.g);
        buf.push(c.b);
    }
    buf
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
    cloud_data: &[f32],
    samples_per_axis: u32,
) -> Vec<f32> {
    let spheres = parse_spheres(sphere_data, diffuse_intensity);
    let triangles = parse_triangles(triangle_data, diffuse_intensity);
    let bvh = Bvh::build(&triangles);
    let lights = parse_lights(light_data);
    let clouds = parse_clouds(cloud_data);
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

    let n = samples_per_axis.max(1);
    let sample_count = (n * n) as f32;
    let inv_n = 1.0 / n as f32;

    for y in start_row..end_row {
        for x in 0..width {
            let flipped_y = height as f32 - y as f32 - 1.0;
            let mut accum = Color::new(0.0, 0.0, 0.0);
            for sy in 0..n {
                for sx in 0..n {
                    let dx = (sx as f32 + 0.5) * inv_n - 0.5;
                    let dy = (sy as f32 + 0.5) * inv_n - 0.5;
                    let color = get_pixel(
                        x as f32 + dx,
                        flipped_y as f32 + dy,
                        width as f32,
                        height as f32,
                        &spheres,
                        &triangles,
                        &bvh,
                        &lights,
                        &camera,
                        skybox_ref,
                        &clouds,
                    );
                    accum += color;
                }
            }
            let avg = accum * (1.0 / sample_count);
            pixels.push(avg.r);
            pixels.push(avg.g);
            pixels.push(avg.b);
            pixels.push(alpha);
        }
    }

    pixels
}
