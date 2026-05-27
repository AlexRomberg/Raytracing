const GRADIENTS_2D: [(f32, f32); 8] = [
    (1.0, 0.0),
    (-1.0, 0.0),
    (0.0, 1.0),
    (0.0, -1.0),
    (0.70710677, 0.70710677),
    (-0.70710677, 0.70710677),
    (0.70710677, -0.70710677),
    (-0.70710677, -0.70710677),
];

fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

fn hash2(ix: i32, iy: i32, seed: u32) -> u32 {
    let mut h = seed;
    h = h.wrapping_mul(2654435761).wrapping_add(ix as u32);
    h = h.wrapping_mul(2654435761).wrapping_add(iy as u32);
    h ^= h >> 13;
    h = h.wrapping_mul(0x5bd1_e995);
    h ^= h >> 15;
    h
}

fn gradient(ix: i32, iy: i32, seed: u32) -> (f32, f32) {
    GRADIENTS_2D[(hash2(ix, iy, seed) & 7) as usize]
}

pub fn perlin_2d(x: f32, y: f32, seed: u32) -> f32 {
    let xi = x.floor() as i32;
    let yi = y.floor() as i32;
    let xf = x - xi as f32;
    let yf = y - yi as f32;

    let u = fade(xf);
    let v = fade(yf);

    let g00 = gradient(xi, yi, seed);
    let g10 = gradient(xi + 1, yi, seed);
    let g01 = gradient(xi, yi + 1, seed);
    let g11 = gradient(xi + 1, yi + 1, seed);

    let d00 = g00.0 * xf + g00.1 * yf;
    let d10 = g10.0 * (xf - 1.0) + g10.1 * yf;
    let d01 = g01.0 * xf + g01.1 * (yf - 1.0);
    let d11 = g11.0 * (xf - 1.0) + g11.1 * (yf - 1.0);

    let x1 = d00 + u * (d10 - d00);
    let x2 = d01 + u * (d11 - d01);
    (x1 + v * (x2 - x1)) * std::f32::consts::SQRT_2
}

pub fn fbm_2d(
    x: f32,
    y: f32,
    octaves: u32,
    persistence: f32,
    lacunarity: f32,
    seed: u32,
) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;
    let mut max_amp = 0.0;

    for _ in 0..octaves {
        value += amplitude * perlin_2d(x * frequency, y * frequency, seed);
        max_amp += amplitude;
        amplitude *= persistence;
        frequency *= lacunarity;
    }

    if max_amp > 0.0 {
        value / max_amp
    } else {
        0.0
    }
}
