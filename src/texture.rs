use crate::color::Color;

#[derive(Serialize, Deserialize)]
pub struct Texture {
    pub image: DynamicImage,
}

// impl Texture {
//     pub fn new(path: String) -> Texture {
//         Texture {
//             tex: image::open(path)
//         }
//     }
// }

fn dummy_texture() -> DynamicImage {
    DynamicImage::new_rgb8(0, 0)
}

fn load_texture<D>(deserializer: D) -> Result<Texture, D::Error> where D: Deserializer, {
    let texture = Texture::deserialize(deserializer)?;
    if let Ok(img) = image::open(texture.path.clone()) {
        Ok(Texture {
            path: texture.path,
            texture: img,
        })
    } else {
        Err(::serde::de::Error::custom(format!(
            "Unable to open texture file: {:?}",
            texture.path
        )))
    }
}
impl fmt::Debug for Texture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Texture({:?})", self.path)
    }
}