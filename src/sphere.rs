use crate::vector3::Vector3;
use crate::ray::Ray;
use crate::color::Color;
use crate::scene::Intersectable;
use crate::material::Material;
use crate::material::TextureCoords;

pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
    pub material: Material
}

impl Sphere {
    pub fn surface_normal(&self, hit_point: &Vector3) -> Vector3 {
        (*hit_point - self.center).normalize()
    }
    pub fn texture_coords(&self, hit_point: &Vector3) -> TextureCoords {
        let hit_vec = *hit_point - self.center;
        TextureCoords {
            x: (1.0 + (hit_vec.z.atan2(hit_vec.x) as f32) / std::f32::consts::PI) * 0.5,
            y: (hit_vec.y / self.radius).acos() as f32 / std::f32::consts::PI,
        }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let l: Vector3 = self.center - ray.origin;
        let adj = l.dot(&ray.direction);
        let d2 = l.dot(&l) - (adj * adj);
        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
            return None;
        }
        let thc = (radius2 - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;
 
        if t0 < 0.0 && t1 < 0.0 {
            return None;
        } else if t0 < 0.0 {
            Some(t1)
        } else if t1 < 0.0 {
            Some(t0)
        } else {
            let distance = if t0 < t1 { t0 } else { t1 };
            Some(distance)
        }
    }
}



// impl Intersectable for Sphere {
//     fn intersect(&self, ray: &Ray) -> bool {
//         //Create a line segment between the ray origin and the center of the sphere
//         let l: Vector3 = self.center.clone() - ray.origin.clone();
//         //Use l as a hypotenuse and find the length of the adjacent side
//         let adj2 = l.dot(&ray.direction);
//         //Find the length-squared of the opposite side
//         //This is equivalent to (but faster than) (l.length() * l.length()) - (adj2 * adj2)
//         let d2 = l.dot(&l) - (adj2 * adj2);
//         //If that length-squared is less than radius squared, the ray intersects the sphere
//         d2 < (self.radius * self.radius)
//     }
// }