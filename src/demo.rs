use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};
use sdl2::surface::Surface;
use std::thread::sleep;
use raytrace::math::{Point, Vec3};
use raytrace::math::Color as Color3;
use raytrace::{Ray, Sphere, Hit, Camera};
use sdl2::render::{Texture, TextureCreator, Canvas};
use sdl2::pixels::{PixelFormatEnum, Color};
use sdl2::video::{Window, WindowContext};
use sdl2::VideoSubsystem;
use core::cmp::{min, max};
//Based on https://raytracing.github.io/books/RayTracingInOneWeekend.html
fn calculate_ray(ray : &Ray, sphere : &Sphere) -> Color3 {
    if let Some(hit) = sphere.hit(ray, 0.0, 1000000.0){
        sphere.color
    } else {
        let unit_dir = ray.direction().normalize();
        let a = 0.5 * (unit_dir.y() + 1.0);
        (Color3::new(1.0, 1.0, 1.0) * (1.0 - a)) + (Color3::new(0.5, 0.7, 1.0) * a)
    } 
}
const BAYER_4 : [[f32; 4]; 4] = [
    [0.0000, 0.5000, 0.1250, 0.6250],
    [0.7500, 0.2500, 0.8750, 0.3750],
    [0.1875, 0.6875, 0.0625, 0.5625],
    [0.9375, 0.4375, 0.8125, 0.3125]
];
//https://github.com/OneLoneCoder/Javidx9/blob/master/PixelGameEngine/SmallerProjects/OneLoneCoder_PGE_Dithering.cpp
fn quantize_n_bit<const N : usize>(color : &Color3) -> (u8, u8, u8) {
    let levels : f32 = ((1 << N) - 1) as f32;
    let r : u8 = unsafe {((color.0[0] * levels).round() / levels * 255.0).min(255.0).max(0.0).round().to_int_unchecked()};
    let g : u8 = unsafe {((color.0[1] * levels).round() / levels * 255.0).min(255.0).max(0.0).round().to_int_unchecked()};
    let b : u8 = unsafe {((color.0[2] * levels).round() / levels * 255.0).min(255.0).max(0.0).round().to_int_unchecked()};
    return (r, g, b)
}
fn quantize_n_bit_ordered_dithering<const N : usize>(color : &Color3, x : u16, y : u16) -> (u8, u8, u8) {
    let levels : f32 = ((1 << N) - 1) as f32;
    let dither_coef = BAYER_4[(y & 3) as usize][(x & 3) as usize];
    let r : u8 = unsafe {((color.0[0] * levels + dither_coef - 0.5).round() / levels * 255.0).min(255.0).max(0.0).round().to_int_unchecked()};
    let g : u8 = unsafe {((color.0[1] * levels + dither_coef - 0.5).round() / levels * 255.0).min(255.0).max(0.0).round().to_int_unchecked()};
    let b : u8 = unsafe {((color.0[2] * levels + dither_coef - 0.5).round() / levels * 255.0).min(255.0).max(0.0).round().to_int_unchecked()};
    return (r, g, b)
}
pub fn main(){
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("raytracer_demo", 640, 480).
            position_centered().
            build().
            unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let camera = Camera::from_viewport(640, 480, 1);
    let framebuffer_builder = canvas.texture_creator();
    let mut framebuffer = framebuffer_builder.create_texture_streaming(PixelFormatEnum::RGB24, camera.width as u32, camera.height as u32).unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut paused : bool = false;
    let sphere = Sphere {
        loc : Point::new(0.0, 0.0, -1.0),
        radius : 0.5,
        color : Color3::new(1.0, 0.0, 0.0)
    };
    'running : loop {
        let loop_start = Instant::now();
        if !paused {
            for event in event_pump.poll_iter(){
                match event {
                    Event::Quit{..} | 
                    Event::KeyDown {keycode : Some(Keycode::Escape), ..} => {
                        println!("Loop Terminating");
                        break 'running;
                    }
                    Event::KeyDown {keycode : Some(Keycode::Space), ..} => {
                        paused = true;
                    }
                    _ => {}
                }
            }
        } else {
            while paused {
                for event in event_pump.poll_iter(){
                    match event {
                        Event::Quit{..} | 
                        Event::KeyDown {keycode : Some(Keycode::Escape), ..} => {
                            println!("Loop Terminating");
                            break 'running;
                        }
                        Event::KeyDown {keycode : Some(Keycode::Space), ..} => {
                            paused = false;
                        }
                        _ => {}
                    }
                }
            }
        }
        let now = Instant::now();
        framebuffer.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..camera.height {
                for x in 0..camera.width {
                    /* let mut color = Rgb::new(0.0, 0.0, 0.0);
                    for sample in 0..self.samples_per_pix {
                        let ray = self.get_ray(x, y);
                        color += calculate_ray(&ray, &spheres);
                    } */
                    let ray = camera.get_ray(x, y);
                    let color = calculate_ray(&ray, &sphere);
                    let offset = (y as usize) * pitch + (x  as usize) * 3;
                    //color /= (self.samples_per_pix as f32);
                    //let rgb =  color.to_rgb();
                    let rgb = quantize_n_bit_ordered_dithering::<2>(&color, x, y);
                    //Safety: Bounds checked by loops
                    unsafe {
                        { 
                            let buf_r = buffer.get_unchecked_mut(offset + 0);
                            *buf_r = rgb.0;
                        }
                        {
                            let buf_g = buffer.get_unchecked_mut(offset + 1);
                            *buf_g = rgb.1;
                        }
                        {
                            let buf_b = buffer.get_unchecked_mut(offset + 2);
                            *buf_b = rgb.2;
                        }
                    }
                }
            }
        });
        let duration = now.elapsed();
        let loop_time = loop_start.elapsed();
        canvas.copy(&framebuffer, None, None).unwrap();
        canvas.present();
        println!("It took {} ms to render the frame", duration.as_millis());
        println!("The loop took {} ms to run", loop_time.as_millis());
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
