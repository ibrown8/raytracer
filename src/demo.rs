use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::surface::Surface;
use std::thread::sleep;
use geometric::{Vec2, Vec3};
use raytrace::calculate_ray;

pub fn main(){
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("raytracer_demo", 640, 480).
        position_centered().
        build().
        unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    println!("Canvas Created");
    let texture_builder = canvas.texture_creator();
    canvas.set_draw_color(Color::RGB(128, 128, 128));
    canvas.clear();
    canvas.present();
    println!("Canvas presented");
    let mut i = 0;
    let mut event_pump = sdl_context.event_pump().unwrap();
    let eye : Vec3<f32> = Vec3{x : 0.0, y : 0.0, z : 0.0};
    let target : Vec3<f32> = Vec3{x : 0.0, y : 200.0, z : 0.0};
    let viewport : Vec2<u32> = Vec2{x : 640, y : 480};
    let d : f32 = 100.0;
    'running : loop {
        //println!("i = { }", i);
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        //println!("Canvas set");
        canvas.clear();
        println!("Canvas cleared");
        for event in event_pump.poll_iter(){
            match event {
                Event::Quit{..} | 
                Event::KeyDown {keycode : Some(Keycode::Escape), ..} => {
                    println!("Loop Terminating");
                    break 'running;
                }
                _ => {}
            }
        }
        for y in 0..480 {
            for x in 0..640 {
                let pixel : Vec2<u32> = Vec2{x, y};
                let ray = calculate_ray(&eye, &target, &viewport, x, y, d);
                println!("ray = ({}, {}, {})", ray.x, ray.y, ray.z);
            }
        }
        canvas.present();
        //println!("Canvas presented");
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    //let surface = Surface::new(640, 480, PixelFormatEnum::RGB24).unwrap();
    //let texture = surface.as_texture(&texture_builder).unwrap()
}
