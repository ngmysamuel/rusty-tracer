mod vector3;
use vector3::Vector3;
mod color;
use color::Color;
mod ray;
use ray::Ray;
mod sphere;
use sphere::Sphere;
mod plane;
use plane::Plane;
mod scene;
use scene::Scene;
use crate::scene::Intersectable;
use scene::Intersection;
mod light;
use light::Light;
use crate::light::DirectionalLight;
use crate::light::SphericalLight;
mod material;
use material::Material;
use image::*;

const BLACK: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
};

fn main() {
    test_can_render_scene();
}

pub fn render(sceneInstance: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(sceneInstance.width, sceneInstance.height);
    let sky_blue = Rgba::from_channels(135, 206, 250, 255);
    let slate_grey = Rgba::from_channels(108, 119, 149, 255);
    for x in 0..sceneInstance.width {
        for y in 0..sceneInstance.height {
            let ray = Ray::create_prime(x, y, sceneInstance);

            let intersection = sceneInstance.trace(&ray);
            match intersection {
                Some(ele) => {
                    let mut color = Color {
                        red: 0.0,
                        blue: 0.0,
                        green: 0.0,
                    };
                    color = get_color(&sceneInstance, &ray, &ele, 0);
                    image.put_pixel(x, y, color.clamp().to_rgba());
                },
                None    => image.put_pixel(x, y, slate_grey),
            }

            // if scene.sphere.intersect(&ray) {
            //     let c = &scene.sphere.color.to_rgba();
            //     image.put_pixel(x, y,  *c)//, to_rgba(&scene.sphere.color)
            // } else {
            //     image.put_pixel(x, y, black);
            // }
        }
    }
    image
}

fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection, depth: u32) -> Color {
    let hit_point = ray.origin + (ray.direction * intersection.distance);
    let surface_normal = intersection.element.surface_normal(&hit_point);

    let mut color = diffuse_color(scene, intersection, &hit_point, &surface_normal); //gets the normal diffuse lighting effect
    if let material::SurfaceType::Reflective { reflectivity } = intersection.element.material().surface { //if the intersection's element's surface type matches a reflective surface
        let reflection_ray = Ray::create_reflection(surface_normal, ray.direction, hit_point, scene.shadow_bias);
        color = color * (1.0 - reflectivity);
        color = color + (cast_ray(scene, &reflection_ray, depth + 1) * reflectivity);
    }
    color
}

fn diffuse_color(sceneInstance: &Scene, ele: &Intersection, hit_point: &Vector3, surface_normal: &Vector3) -> Color {
    let mut color = Color {
        red: 0.0,
        blue: 0.0,
        green: 0.0,
    };
    for light in &sceneInstance.lights {
        let direction_to_light = light.direction_to_light(hit_point); 
        let shadow_ray = Ray {
            origin: *hit_point + (direction_to_light * sceneInstance.shadow_bias),
            direction: direction_to_light,
        };
        let shadow_intersection = sceneInstance.trace(&shadow_ray);
        // if (x > 250 && x < 290) && (y > 160) {
        //     match &shadow_intersection {
        //         Some(blocker) => {println!("At: {} Blocked by: {}", ele.element.obj_str(), blocker.element.obj_str())},
        //         None => {println!("No block")}
        //     }
        // }
        let in_light = light.in_light(shadow_intersection, light.distance(hit_point)); 
        let light_intensity = if in_light {
             light.intensity(&hit_point)
        } else { 0.0 };
        let light_power = (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;
        // if (x > 250 && x < 290) && (y > 160) {
        //     println!("Power: {}, Intent: {}", light_power, light_intensity)
        // }
        let light_reflected = ele.element.albedo() / std::f32::consts::PI;
        let light_color = light.color() * light_power * light_reflected;
        color = color + (ele.element.color(&hit_point) * light_color);
        // if (x > 250 && x < 290) && (y > 160) {
        //     color = Color {
        //         red: 0.0,
        //         blue: 1.0,
        //         green: 0.0,
        //     };
        // }
    }
    color
}

pub fn cast_ray(scene: &Scene, ray: &Ray, depth: u32) -> Color {
    if depth >= scene.max_recursion_depth {
        return BLACK;
    }

    let intersection = scene.trace(&ray);
    intersection.map(|i| get_color(scene, &ray, &i, depth))
        .unwrap_or(BLACK)
}

fn test_can_render_scene() {
    let scene = Scene {
        width: 800, //800 OR 320,
        height: 600, //600 OR 200,
        fov: 90.0,
        elements: vec! [ 
            scene::Element::Sphere(Sphere { // z: move away from camera (-). x: Left (-). y: up (+)
                center: Vector3 {
                    x: 1.0,
                    y: 1.0,
                    z: -2.5,
                },
                radius: 1.0,
                material: Material {
                    coloration: material::Coloration::Texture( image::open(String::from("C:/Users/samue/Documents/rust-tracer/checkerboard.png")).unwrap()  ),
                    albedo: 0.3,
                    surface: material::SurfaceType::Reflective {reflectivity: 0.3}
                }
            } ), 
            scene::Element::Sphere(Sphere {
                center: Vector3 {
                    x: -2.0,
                    y: 0.5,
                    z: -3.5, //-2.5
                },
                radius: 1.0,
                material: Material {
                    coloration: material::Coloration::Texture( image::open(String::from("C:/Users/samue/Documents/rust-tracer/checkerboard-2.png")).unwrap()  ),
                    albedo: 0.3,
                    surface: material::SurfaceType::Diffuse
                } 
            } ),
            scene::Element::Plane(Plane {
                p0: Vector3 {
                    x: -1.0,
                    y: -1.0,
                    z: -1.0,
                },
                normal: Vector3 {
                    x: 0.0,
                    y: -1.0,
                    z: 0.0,
                },
                material: Material {
                    coloration: material::Coloration::Color( Color {
                        red: 0.5,
                        green: 0.5,
                        blue: 0.5,
                    }),
                    albedo: 0.3,
                    surface: material::SurfaceType::Diffuse
                }
            } ),
            scene::Element::Plane(Plane {
                p0: Vector3 {
                    x: -1.0,
                    y: -1.0,
                    z: -5.0,
                },
                normal: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                },
                material: Material {
                    coloration: material::Coloration::Color( Color {
                        red: 0.5,
                        green: 0.5,
                        blue: 0.5,
                    }),
                    albedo: 0.3,
                    surface: material::SurfaceType::Reflective {reflectivity: 0.3}
                }
            } ),
        ], 
        lights: vec ! [
            light::Light::Directional(DirectionalLight { // x: Left (+). y: up (-) z: move away from camera (+). 
                direction: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                },
                intensity: 1.0,
                color: Color {
                    red: 1.0,
                    green: 1.0,
                    blue: 1.0,
                },
            } ),
            light::Light::Spherical(SphericalLight { // x: Left (-). y: up (+) z: move away from camera (+). 
                position: Vector3 {
                    x: -2.0,//3.5,
                    y: 10.0,//2.0,
                    z: -3.5//-2.5,
                },
                intensity: 10000.0,
                color: Color {
                    red: 1.0,
                    green: 1.0,
                    blue: 1.0,
                },
            } ),
            light::Light::Spherical(SphericalLight { // x: Left (-). y: up (+) z: move away from camera (+). 
                position: Vector3 {
                    x: 0.25,
                    y: 0.0,
                    z: -2.0,
                },
                intensity: 150.0,
                color: Color {
                    red: 1.0,
                    green: 1.0,
                    blue: 0.5,
                },
            } )
        ],
        shadow_bias: 0.000000001,
        max_recursion_depth: 5,
    };

    let img: DynamicImage = render(&scene);
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());

    let version = img.save("./test.png");
    match version {
        Ok(v) => println!("Alright! {:?}", v),
        Err(e) => println!("error parsing header: {:?}", e),
    }
}




// match light {
//     Light::Directional(DirectionalLight) => {
//         let hit_point = ray.origin + (ray.direction * ele.distance);
//         let surface_normal = ele.element.surface_normal(&hit_point);
//         let direction_to_light = -light.direction;
//         let shadow_ray = Ray {
//             origin: hit_point + (direction_to_light * sceneInstance.shadow_bias),
//             direction: direction_to_light,
//         };
//         let in_light = sceneInstance.trace(&shadow_ray).is_none();
//         let light_intensity = if in_light { light.intensity } else { 0.0 };
//         let light_power = (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;
//         let light_reflected = ele.element.albedo() / std::f32::consts::PI;
//         let light_color = light.color * light_power * light_reflected;
//         color = color + (ele.element.color() * light_color);
//     },
//     Light::Spherical(SphericalLight) => {
//         let hit_point = ray.origin + (ray.direction * ele.distance);
//         let surface_normal = ele.element.surface_normal(&hit_point);
//         let direction_to_light =  (light.position - *hit_point).norm() as f32;
//         let shadow_ray = Ray {
//             origin: hit_point + (direction_to_light * sceneInstance.shadow_bias),
//             direction: direction_to_light,
//         };
//         let shadow_intersection = sceneInstance.trace(&shadow_ray);
//         let in_light = shadow_intersection.is_none() || shadow_intersection.unwrap().distance > light.distance(&hit_point);
//         let light_intensity = if in_light {
//             light.intensity / (4.0 * ::std::f32::consts::PI * direction_to_light) 
//         } else { 0.0 };
//         let light_power = (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;
//         let light_reflected = ele.element.albedo() / std::f32::consts::PI;
//         let light_color = light.color * light_power * light_reflected;
//         color = color + (ele.element.color() * light_color);
//     }
// }

// }