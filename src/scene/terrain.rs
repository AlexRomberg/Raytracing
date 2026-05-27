use crate::util::color::Color;
use crate::util::noise::fbm_2d;
use crate::util::vector::Vec3;

pub struct TerrainConfig {
    pub grid_size: u32,
    pub width: f32,
    pub depth: f32,
    pub height_scale: f32,
    pub octaves: u32,
    pub persistence: f32,
    pub lacunarity: f32,
    pub seed: u32,
}

pub struct TerrainMesh {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<[u32; 3]>,
    pub face_colors: Vec<Color>,
}

fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    Color::new(
        a.r + (b.r - a.r) * t,
        a.g + (b.g - a.g) * t,
        a.b + (b.b - a.b) * t,
    )
}

fn height_color(h: f32) -> Color {
    let h = h.clamp(0.0, 1.0);
    if h < 0.45 {
        let t = h / 0.45;
        lerp_color(Color::new(0.10, 0.32, 0.10), Color::new(0.32, 0.55, 0.18), t)
    } else if h < 0.55 {
        let t = (h - 0.45) / 0.10;
        lerp_color(Color::new(0.32, 0.55, 0.18), Color::new(0.40, 0.40, 0.40), t)
    } else if h < 0.80 {
        let t = (h - 0.55) / 0.25;
        lerp_color(Color::new(0.40, 0.40, 0.40), Color::new(0.68, 0.68, 0.70), t)
    } else {
        let t = ((h - 0.80) / 0.20).min(1.0);
        lerp_color(Color::new(0.78, 0.78, 0.82), Color::new(1.0, 1.0, 1.0), t)
    }
}

pub fn generate(config: &TerrainConfig) -> TerrainMesh {
    let n = config.grid_size.max(2);
    let denom = (n - 1) as f32;
    let total = (n * n) as usize;

    let mut heights = Vec::with_capacity(total);
    let mut min_h = f32::INFINITY;
    let mut max_h = f32::NEG_INFINITY;

    for y in 0..n {
        for x in 0..n {
            let nx = x as f32 / denom - 0.5;
            let ny = y as f32 / denom - 0.5;
            let h = fbm_2d(
                nx * 4.0,
                ny * 4.0,
                config.octaves.max(1),
                config.persistence,
                config.lacunarity,
                config.seed,
            );
            if h < min_h {
                min_h = h;
            }
            if h > max_h {
                max_h = h;
            }
            heights.push(h);
        }
    }

    let range = (max_h - min_h).max(1e-6);

    let mut vertices = Vec::with_capacity(total);
    for y in 0..n {
        for x in 0..n {
            let i = (y * n + x) as usize;
            let nx = x as f32 / denom - 0.5;
            let ny = y as f32 / denom - 0.5;
            vertices.push(Vec3::new(
                nx * config.width,
                heights[i] * config.height_scale,
                ny * config.depth,
            ));
        }
    }

    let cell_count = ((n - 1) * (n - 1)) as usize;
    let mut faces = Vec::with_capacity(cell_count * 2);
    let mut face_colors = Vec::with_capacity(cell_count * 2);

    let normalize = |h: f32| (h - min_h) / range;

    for y in 0..(n - 1) {
        for x in 0..(n - 1) {
            let i00 = y * n + x;
            let i10 = y * n + (x + 1);
            let i01 = (y + 1) * n + x;
            let i11 = (y + 1) * n + (x + 1);

            faces.push([i00, i10, i11]);
            let h_avg = (heights[i00 as usize] + heights[i10 as usize] + heights[i11 as usize]) / 3.0;
            face_colors.push(height_color(normalize(h_avg)));

            faces.push([i00, i11, i01]);
            let h_avg = (heights[i00 as usize] + heights[i11 as usize] + heights[i01 as usize]) / 3.0;
            face_colors.push(height_color(normalize(h_avg)));
        }
    }

    TerrainMesh {
        vertices,
        faces,
        face_colors,
    }
}
