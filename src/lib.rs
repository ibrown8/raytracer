pub mod math;
//use rayon::iter::ParallelIterator;
//use rayon::slice::ChunksExactMut;
//based on https://raytracing.github.io/books/RayTracingInOneWeekend.html
use math::*;
pub use math::Point;
pub use math::Vec3;
pub use math::Color as Rgb;
pub struct Ray {
    origin : Point,
    direction : Vec3
}
impl Ray {
    pub fn new(origin : &Point, direction : &Vec3) -> Self {
        Self {
            origin : *origin,
            direction : *direction
        }
    }
    #[inline]
    pub fn origin(&self) -> Point {
        self.origin
    }
    #[inline]
    pub fn direction(&self) -> Vec3 {
        self.direction
    }
    #[inline]
    pub fn point_at(&self, t: f32) -> Point {
        self.origin + self.direction * t
    }
}
#[derive(Clone, Copy)]
pub struct HitRecord {
    pub point : Point,
    pub normal : Vec3,
    pub t : f32,
    pub front_face : bool
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray : &Ray, out_normal : &Vec3){
        self.front_face = ray.direction().dot(out_normal) < 0.0;
        self.normal = if self.front_face {
            *out_normal
        } else {
            -(*out_normal)
        };
    }
}
fn calculate_ray(ray : &Ray, sphere : &Sphere) -> Rgb {
    if let Some(hit) = sphere.hit(ray, 0.0, 1000000.0){
        sphere.color
    } else {
        let unit_dir = ray.direction().normalize();
        let a = 0.5 * (unit_dir.y() + 1.0);
        (Rgb::new(1.0, 1.0, 1.0) * (1.0 - a)) + (Rgb::new(0.5, 0.7, 1.0) * a)
    } 
}
pub trait Hit {
    fn hit(&self, ray : &Ray, ray_min : f32, ray_max : f32) -> Option<HitRecord>;
}
#[derive(Clone, Copy)]
pub struct Sphere {
    pub loc : Point,
    pub radius : f32,
    pub color : Rgb
}  

pub struct Camera {
    pub width : u16,
    pub height : u16,
    pub focal_len : f32, 
    pub center : Point,
    pub pixel_delta_u : Vec3,
    pub pixel_delta_v : Vec3,
    pub viewport_upper_left : Vec3,
    pub pixel00_loc : Vec3,
    pub samples_per_pix : u8
}
impl Camera {
    pub fn from_viewport(width : u16, height : u16, samples_per_pix : u8) -> Self {
        let width_f = width as f32;
        let height_f = height as f32;
        let aspect_ratio = width_f / height_f;
        let focal_len = 1.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let center = Point::new(0.0, 0.0, 0.0);
        //Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        let pixel_delta_u = viewport_u / width_f;
        let pixel_delta_v = viewport_v / height_f;
        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let viewport_upper_left = center - Vec3::new(0.0, 0.0, focal_len) - viewport_u/2.0 - viewport_v/2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;
        Self {width,  height, focal_len, center, 
            pixel_delta_u, pixel_delta_v, viewport_upper_left, pixel00_loc, samples_per_pix
        }
    }
    #[inline]
    pub fn get_ray(&self, x : u16, y : u16) -> Ray {
        let pixel_center = self.pixel00_loc + (self.pixel_delta_u * (x as f32)) + (self.pixel_delta_v * (y as f32));
        let ray_origin = self.center;
        let ray_direction = pixel_center - ray_origin;
        return Ray::new(&ray_origin, &ray_direction)
    }
    pub fn get_ray2(&self, x : u16, y : u16) -> Ray {
        let pixel_center = self.pixel00_loc + (self.pixel_delta_u * (x as f32)) + (self.pixel_delta_v * (y as f32));
        let pixel_sample = pixel_center + self.pixel_sample_square();
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        return Ray::new(&ray_origin, &ray_direction)
    }
    #[inline]
    fn pixel_sample_square(&self) -> math::Vec3 {
        let px : f32 = -0.5 + rand::random::<f32>();
        let py : f32 = -0.5 + rand::random::<f32>();
        return (self.pixel_delta_u * px) + (self.pixel_delta_v * py)
    }
}

pub struct SphereList {
    pub spheres : Vec<Sphere>
}


    /*pub fn render_parallel(&mut self, &Sphere){
        self.framebuffer.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            (0..self.height).into_par_iter().zip(buffer.chuncks_exact_mut(pitch)).for_each(|y, scanline| {
                for x in 0..self.width {
                    let mut color = Color::new(0.0, 0.0, 0.0);
                    for sample in 0..self.samples_per_pix {
                        let ray = self.get_ray(x, y);
                        color += calculate_ray(&ray, sphere);
                    }
                    let offset = (x  as usize) * 3;
                    color /= (samples_per_pix as f32);
                    let rgb =  color.to_rgb();
                    unsafe {
                        let buf_r = scanline.get_unchecked_mut(offset + 0);
                        *buf_r = color.r;
                        let buf_g = scanline.get_unchecked_mut(offset + 1);
                        *buf_g = color.g;
                        let buf_b = scanline.get_unchecked_mut(offset + 2);
                        *buf_r = color.b;
                    }
                }
            })
        });
    } */
    

impl Hit for Sphere {
    fn hit(&self, ray : &Ray, ray_min : f32, ray_max : f32) -> Option<HitRecord> {
        let origin_center = ray.origin() - self.loc;
        let a = ray.direction().dot(&ray.direction());
        let half_b = origin_center.dot(&ray.direction());
        let c = origin_center.len_squared() - self.radius * self.radius;
        let discrim = half_b * half_b - a * c;
        if discrim < 0.0 {
            return None
        } 
        let root_d = discrim.sqrt();
        let mut root = (-half_b - root_d) / a;
        if root <= ray_min || ray_max <= root {
            root = (-half_b + root_d) / a;
            if root <= ray_min || ray_max <= root {
                return None
            }
        }
        let point = ray.point_at(root);
        let out_normal = (point - self.loc) / self.radius;
        let mut result = HitRecord {
            point, normal : out_normal, t : root, front_face : true 
        };
        result.set_face_normal(ray, &out_normal);
        Some(result)
    }
}
impl Hit for SphereList {
    fn hit(&self, ray : &Ray, ray_min : f32, ray_max : f32) -> Option<HitRecord> {
        let mut return_val : Option<HitRecord> = None;
        let mut closest = ray_max;
        for sphere in &self.spheres {
            if let Some(hit) = sphere.hit(ray, ray_min, closest){
                return_val = Some(hit);
                closest = hit.t;
            }
        }
        return return_val;
    }
}
use std::f32::consts::PI;
//use sdl2::pixels::Color;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
