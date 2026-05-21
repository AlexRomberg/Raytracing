use crate::util::color::Color;
use crate::util::vector::Vec3;
use std::f32::consts::PI;

pub struct Skybox<'a> {
    pub pixels: &'a [f32],
    pub width: u32,
    pub height: u32,
    pub brightness: f32,
}

impl<'a> Skybox<'a> {
    pub fn from_slice(pixels: &'a [f32], width: u32, height: u32, brightness: f32) -> Option<Self> {
        if width == 0 || height == 0 || pixels.len() < (width * height * 3) as usize {
            return None;
        }
        Some(Self {
            pixels,
            width,
            height,
            brightness,
        })
    }

    pub fn sample(&self, direction: Vec3) -> Color {
        let d = direction.normalized();
        let u = 0.5 + d.z.atan2(d.x) / (2.0 * PI);
        let v = 0.5 - d.y.clamp(-1.0, 1.0).asin() / PI;

        let x = (u.fract() + 1.0).fract() * self.width as f32;
        let y = v.clamp(0.0, 1.0) * (self.height as f32 - 1.0);

        let x0 = x.floor() as i32;
        let y0 = y.floor() as i32;
        let x1 = (x0 + 1).rem_euclid(self.width as i32);
        let y1 = (y0 + 1).min(self.height as i32 - 1);
        let fx = x - x0 as f32;
        let fy = y - y0 as f32;

        let c00 = self.fetch(x0, y0);
        let c10 = self.fetch(x1, y0);
        let c01 = self.fetch(x0, y1);
        let c11 = self.fetch(x1, y1);

        let top = c00 * (1.0 - fx) + c10 * fx;
        let bot = c01 * (1.0 - fx) + c11 * fx;
        (top * (1.0 - fy) + bot * fy) * self.brightness
    }

    fn fetch(&self, x: i32, y: i32) -> Color {
        let idx = ((y as u32 * self.width + x as u32) * 3) as usize;
        Color::new(self.pixels[idx], self.pixels[idx + 1], self.pixels[idx + 2])
    }
}
