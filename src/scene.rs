use crate::vector3::Vector3;
use crate::sphere::Sphere;
use crate::plane::Plane;
use crate::ray::Ray;
use crate::color::Color;
use crate::light::Light;
use crate::material::TextureCoords;
use crate::material::Material;
use serde::{Serialize, Deserialize};

#[derive(Clone, Deserialize)]
pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
}

impl Element {
    //PROBLEM HERE: easy fix, just inherit the Copy trait on Color. Hard way: do not use the copy trait
    //It might not be so bad to implement Copy.
    //We will be consuming the Copy-ed object when we Mul operator in Color. 
    //We must make sure the field: "color" in Sphere/Plane is always the owner of the Color object. (why? well, the naive reason is that we simply
    //cannot change the owner, there are other references to that owner.)
    //There are only 2 ways to make sure of that here: by reference or creating a copy
    pub fn color(&self, hit_point: &Vector3) -> Color {
        match *self {
            Element::Sphere(ref s) => {
                s.material.coloration.color( &s.texture_coords(hit_point) )
            },
            Element::Plane(ref p) => {
                p.material.coloration.color( &p.texture_coords(hit_point) )
            }
        }
    }
    pub fn albedo(&self) -> f32 {
        match *self {
            Element::Sphere(ref s) => s.material.albedo,
            Element::Plane(ref p) => p.material.albedo,
        }
    }
    pub fn material(&self) -> &Material {
        match *self {
            Element::Sphere(ref s) => &s.material,
            Element::Plane(ref p) => &p.material,
        }
    }
    pub fn surface_normal(&self, hit_point: &Vector3) -> Vector3 {
        match *self {
            Element::Sphere(ref s) => s.surface_normal(hit_point),
            Element::Plane(ref p) => p.surface_normal(hit_point),
        }
    }
    pub fn texture_coords(&self, hit_point: &Vector3) -> TextureCoords {
        match *self {
            Element::Sphere(ref s) => s.texture_coords(hit_point),
            Element::Plane(ref p) => p.texture_coords(hit_point),
        }
    }
    pub fn obj_str(&self) -> &str {
        match *self {
            Element::Sphere(ref s) => "Sphere",
            Element::Plane(ref p) => "Plane",
        }
    }
    
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
}

impl Intersectable for Element {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match *self {
            Element::Sphere(ref s) => s.intersect(ray),
            Element::Plane(ref p) => p.intersect(ray),
        }
    }
}


#[derive(Clone, Deserialize)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub elements: Vec<Element>,
    pub lights: Vec<Light>,
    pub shadow_bias: f64,
    pub max_recursion_depth: u32,
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.elements
            .iter()
            .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}


pub struct Intersection<'a> {
    pub distance: f64,
    pub element: &'a Element,
    //Prevent outside code from constructing this; should use the new method and check the distance.
    _secret: (),
}

impl<'a> Intersection<'a> {
    pub fn new<'b>(distance: f64, element: &'b Element) -> Intersection<'b> {
        if !distance.is_finite() {
            panic!("Intersection must have a finite distance.");
        }
        Intersection {
            distance: distance,
            element: element,
            _secret: (),
        }
    }
}