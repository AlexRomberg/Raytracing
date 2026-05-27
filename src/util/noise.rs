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

const GRADIENTS_3D: [(f32, f32, f32); 12] = [
    (1.0, 1.0, 0.0),
    (-1.0, 1.0, 0.0),
    (1.0, -1.0, 0.0),
    (-1.0, -1.0, 0.0),
    (1.0, 0.0, 1.0),
    (-1.0, 0.0, 1.0),
    (1.0, 0.0, -1.0),
    (-1.0, 0.0, -1.0),
    (0.0, 1.0, 1.0),
    (0.0, -1.0, 1.0),
    (0.0, 1.0, -1.0),
    (0.0, -1.0, -1.0),
];

fn hash3(ix: i32, iy: i32, iz: i32, seed: u32) -> u32 {
    let mut h = seed;
    h = h.wrapping_mul(2654435761).wrapping_add(ix as u32);
    h = h.wrapping_mul(2654435761).wrapping_add(iy as u32);
    h = h.wrapping_mul(2654435761).wrapping_add(iz as u32);
    h ^= h >> 13;
    h = h.wrapping_mul(0x5bd1_e995);
    h ^= h >> 15;
    h
}

fn gradient_3d(ix: i32, iy: i32, iz: i32, seed: u32) -> (f32, f32, f32) {
    GRADIENTS_3D[(hash3(ix, iy, iz, seed) % 12) as usize]
}

pub fn perlin_3d(x: f32, y: f32, z: f32, seed: u32) -> f32 {
    let xi = x.floor() as i32;
    let yi = y.floor() as i32;
    let zi = z.floor() as i32;
    let xf = x - xi as f32;
    let yf = y - yi as f32;
    let zf = z - zi as f32;

    let u = fade(xf);
    let v = fade(yf);
    let w = fade(zf);

    let dot = |g: (f32, f32, f32), fx: f32, fy: f32, fz: f32| g.0 * fx + g.1 * fy + g.2 * fz;

    let n000 = dot(gradient_3d(xi, yi, zi, seed), xf, yf, zf);
    let n100 = dot(gradient_3d(xi + 1, yi, zi, seed), xf - 1.0, yf, zf);
    let n010 = dot(gradient_3d(xi, yi + 1, zi, seed), xf, yf - 1.0, zf);
    let n110 = dot(gradient_3d(xi + 1, yi + 1, zi, seed), xf - 1.0, yf - 1.0, zf);
    let n001 = dot(gradient_3d(xi, yi, zi + 1, seed), xf, yf, zf - 1.0);
    let n101 = dot(gradient_3d(xi + 1, yi, zi + 1, seed), xf - 1.0, yf, zf - 1.0);
    let n011 = dot(gradient_3d(xi, yi + 1, zi + 1, seed), xf, yf - 1.0, zf - 1.0);
    let n111 = dot(gradient_3d(xi + 1, yi + 1, zi + 1, seed), xf - 1.0, yf - 1.0, zf - 1.0);

    let nx00 = n000 + u * (n100 - n000);
    let nx10 = n010 + u * (n110 - n010);
    let nx01 = n001 + u * (n101 - n001);
    let nx11 = n011 + u * (n111 - n011);
    let nxy0 = nx00 + v * (nx10 - nx00);
    let nxy1 = nx01 + v * (nx11 - nx01);
    nxy0 + w * (nxy1 - nxy0)
}

pub fn fbm_3d(
    x: f32,
    y: f32,
    z: f32,
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
        value += amplitude * perlin_3d(x * frequency, y * frequency, z * frequency, seed);
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
