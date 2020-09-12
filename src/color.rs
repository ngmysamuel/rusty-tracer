// use image;
use std::ops::{Mul, Add};
use image::{Pixel, Rgba};
use serde::{Serialize, Deserialize};

const GAMMA: f32 = 2.2;

fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0 / GAMMA)
}

fn gamma_decode(encoded: f32) -> f32 {
    encoded.powf(GAMMA)
}

#[derive(Clone, Debug, Deserialize, Serialize, Copy)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {

    pub fn clamp(&self) -> Color {
        Color {
            red: self.red.min(1.0).max(0.0),
            blue: self.blue.min(1.0).max(0.0),
            green: self.green.min(1.0).max(0.0),
        }
    }

    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba::from_channels(
            (gamma_encode(self.red) * 255.0) as u8,
            (gamma_encode(self.green) * 255.0) as u8,
            (gamma_encode(self.blue) * 255.0) as u8,
            255,
        )
    }

    pub fn from_rgba(rgba: Rgba<u8>) -> Color {
        Color {
            red: gamma_decode((rgba[0] as f32) / 255.0),
            green: gamma_decode((rgba[1] as f32) / 255.0),
            blue: gamma_decode((rgba[2] as f32) / 255.0),
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Color {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs
        }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Color) -> Color {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue
        }
    }
}

// impl Mul<f64> for Color {
//     type Output = Self;

//     fn mul(self, rhs: f64) -> Self::Output {
//         Color {
//             x: self.x * rhs,
//             y: self.y * rhs,
//             z: self.z * rhs
//         }
//     }
// }