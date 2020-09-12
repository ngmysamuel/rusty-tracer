use crate::color::Color;
// use crate::texture::Texture;
use image::{DynamicImage, GenericImageView, Pixel, Rgba, ImageBuffer};
use serde::{Serialize, Deserialize};
use serde::de::{self, Deserializer, Visitor, SeqAccess, MapAccess, IntoDeserializer};
use std::fmt;
extern crate base64;

#[derive(Clone, Deserialize)]
pub struct Material {
    pub coloration: Coloration,
    pub albedo: f32,
    pub surface: SurfaceType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SurfaceType {
    Diffuse,
    Reflective { reflectivity: f32 },
    Refractive { index: f32, transparency: f32 },
}

#[derive(Clone)]
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

impl<'de> Deserialize<'de> for Coloration {
    fn deserialize<D>(deserializer: D) -> Result<Coloration, D::Error>
    where
        D: Deserializer<'de>,
    {

        enum Field { Color, Texture };

        // This part could also be generated independently by:
        //
        //    #[derive(Deserialize)]
        //    #[serde(field_identifier, rename_all = "lowercase")]
        //    enum Field { Secs, Nanos }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`secs` or `nanos`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "Color" => Ok(Field::Color),
                            "Texture" => Ok(Field::Texture),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct ColorationVisitor;

        impl<'de> Visitor<'de> for ColorationVisitor {
            type Value = Coloration;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("A map of enum type to the value")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Coloration, V::Error> where V: MapAccess<'de>, {
                let mut color = None;
                let mut texture: std::option::Option<String> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Color => {
                            if color.is_some() {
                                return Err(de::Error::duplicate_field("color"));
                            }
                            color = Some(map.next_value()?);
                        }
                        Field::Texture => {
                            if texture.is_some() {
                                return Err(de::Error::duplicate_field("texture"));
                            }
                            texture = Some(map.next_value()?);
                        }
                    }
                }
                match color {
                    Some(inner) => Ok(Coloration::Color(inner)),
                    None => {
                        match texture {
                            Some(inner) => {
                                let bytes = base64::decode(inner).unwrap();
                                // println!("bytes[0]: {}", bytes[0]);
                                let img: DynamicImage = image::load_from_memory(&bytes).unwrap();
                                Ok(Coloration::Texture(img))
                            },
                            None => Err(de::Error::missing_field("color or text"))
                        }
                    }
                }
            }
        }
        const FIELDS: &'static [&'static str] = &["Color", "Texture"];
        deserializer.deserialize_struct("Coloration", FIELDS, ColorationVisitor)
    }
}

pub struct TextureCoords {
    pub x: f32,
    pub y: f32,
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