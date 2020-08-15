use crate::vector3::Vector3;
use crate::color::Color;
// use crate::point::Point;
use crate::scene::Intersection;

pub enum Light {
    Directional(DirectionalLight),
    Spherical(SphericalLight),
}

pub struct DirectionalLight {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}

pub struct SphericalLight {
    pub position: Vector3,
    pub color: Color,
    pub intensity: f32,
}

impl Light {
    pub fn direction_to_light(&self, hit_point: &Vector3) -> Vector3 {
        match *self {
            Light::Directional(ref d) => {
                -d.direction.normalize()
            },
            Light::Spherical(ref s) => {
                (s.position - *hit_point).normalize() 
            },
        }
    }

    pub fn in_light(&self, shadow_intersection: Option<Intersection>, distance: f64) -> bool {
        match *self {
            Light::Directional(ref d) => {
                shadow_intersection.is_none()
            },
            Light::Spherical(ref s) => {
                shadow_intersection.is_none() || shadow_intersection.unwrap().distance > distance
            },
        }
    }

    pub fn intensity(&self, hit_point: &Vector3) -> f32 {
        match *self {
            Light::Directional(ref d) => {
                d.intensity
            },
            Light::Spherical(ref s) => {
                let r2 = (s.position - *hit_point).dot(&(s.position - *hit_point)) as f32;
                s.intensity / ( r2 * ::std::f32::consts::PI * 4.0)
            },
        }
    }

    pub fn distance(&self, hit_point: &Vector3) -> f64 {
        match *self {
            Light::Directional(_) => ::std::f64::INFINITY,
            Light::Spherical(ref s) => (s.position - *hit_point).length(),
        }
    }

    pub fn color(&self) -> Color {
        match *self {
            Light::Directional(ref d) => {d.color},
            Light::Spherical(ref s) => {s.color},
        }
    }

}