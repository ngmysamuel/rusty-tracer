use crate::vector3::Vector3;
use crate::ray::Ray;
use crate::color::Color;
use crate::scene::Intersectable;
use crate::material::Material;
use crate::material::TextureCoords;
use serde::{Serialize, Deserialize};

#[derive(Clone, Deserialize)]
pub struct Plane {
    pub p0: Vector3,
    pub normal: Vector3,
    pub material: Material
}

impl Plane {
    pub fn surface_normal(&self, _: &Vector3) -> Vector3 {
        -self.normal
    }

    pub fn texture_coords(&self, hit_point: &Vector3) -> TextureCoords  {
        let mut x_axis = self.normal.cross(&Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        });
        if x_axis.length() == 0.0 {
            x_axis = self.normal.cross(&Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            });
        }
        let y_axis = self.normal.cross(&x_axis);
        let hit_vec = *hit_point - self.p0;
        TextureCoords {
            x: hit_vec.dot(&x_axis) as f32,
            y: hit_vec.dot(&y_axis) as f32,
        }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
        if denom > 1e-6 {
            let v = self.p0 - ray.origin;
            let distance = v.dot(&normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }
}