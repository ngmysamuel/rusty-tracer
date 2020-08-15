use crate::color::Color;
// use crate::texture::Texture;
use image::{DynamicImage, GenericImageView, Pixel, Rgba};

pub struct Material {
    pub coloration: Coloration,
    pub albedo: f32,
}

pub struct TextureCoords {
    pub x: f32,
    pub y: f32,
}

pub enum Coloration {
    Color(Color),
    Texture(DynamicImage)
}

impl Coloration {
    pub fn color(&self, texture_coords: &TextureCoords) -> Color {
        match self {
            Coloration::Color(c) => *c,
            Coloration::Texture(tex) => {
                let tex_x = wrap(texture_coords.x, tex.width());
                let tex_y = wrap(texture_coords.y, tex.height());
            
                Color::from_rgba(tex.get_pixel(tex_x, tex_y))
            }
        }
    }
}

fn wrap(val: f32, bound: u32) -> u32 { //to make sure that we do not go outside the sample texture file we are using as our texture i.e. DynamicImage
    let signed_bound = bound as i32;
    let float_coord = val * bound as f32;
    let wrapped_coord = (float_coord as i32) % signed_bound;
    if wrapped_coord < 0 {
        (wrapped_coord + signed_bound) as u32
    } else {
        wrapped_coord as u32
    }
}