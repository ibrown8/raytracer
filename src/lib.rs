pub mod math;
//based on https://raytracing.github.io/books/RayTracingInOneWeekend.html
use math::*;
pub struct Ray {
    origin : math::Point,
    direction : math::Vec3
}
impl Ray {
    pub fn new(origin : &Point, direction : &Vec3) -> Self {
        Self {
            origin : *origin,
            direction : *direction
        }
    }
    #[inline]
    pub fn origin(&self) -> math::Point {
        self.origin
    }
    #[inline]
    pub fn direction(&self) -> math::Vec3 {
        self.direction
    }
    #[inline]
    pub fn point_at(&self, t: f32) -> math::Point {
        self.origin + self.direction * t
    }
}

pub struct HitRecord {
    pub point : math::Point,
    pub normal : math::Vec3,
    pub t : f32,
    pub front_face : bool
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray : &Ray, out_normal : &math::Vec3){
        self.front_face = ray.direction().dot(out_normal) < 0.0;
        self.normal = if self.front_face {
            *out_normal
        } else {
            -(*out_normal)
        };
    }
}
pub trait Hit {
    fn hit(&self, ray : &Ray, ray_min : f32, ray_max : f32) -> Option<HitRecord>;
}

pub struct Sphere {
    pub loc : Point,
    pub radius : f32,
    pub color : Color
} 

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
