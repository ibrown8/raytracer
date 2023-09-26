use geometric::{Vec3, Vec2, Cross};
use std::f32::consts::PI;
use sdl2::pixels::Color;
pub struct Sphere {
    pub loc : Vec3<f32>,
    pub radius : f32,
    pub color : Color
} 
//https://en.wikipedia.org/wiki/Ray_tracing_(graphics)
pub fn calculate_ray(eye : &Vec3<f32>, target : &Vec3<f32>, viewport : &Vec2<u32>, i : u32, j : u32, d : f32) -> Vec3<f32> {
    let v = Vec3{x : 0.0, y : 1.0, z : 0.0};
    let t_n = (target - eye).normalize();
    let b_n = t_n.cross(&v);
    let g_x = d * (PI/4.0).tan();
    let viewport_x = viewport.x as f32;
    let viewport_y = viewport.y as f32;
    let g_y = g_x * viewport_y / viewport_x;
    let q_x = b_n * ((2.0 * g_x) / (viewport_x)); //pixel_shift_x
    let q_y = v * ((2.0 * g_y) / (viewport_y)); //pixel_shift_y
    let p_bottom_left = (t_n * d) - (b_n * g_x) - (v * g_y);
    let i = i as f32;
    let j = j as f32;
    let p = p_bottom_left + (q_x * i) + (q_y * j);
    p.normalize()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
