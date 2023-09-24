use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
//use std::time::Duration;
use sdl::surface::Surface;

pub fn main(){
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("raytracer_demo", 640, 480).
        position_centered().
        build().
        unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_builder = canvas.texture_creator();
    //canvas.set_draw_color(Color::RGBB(128, 128, 128));
    //canvas.present();
    let surface = Surface::new(640, 480, PixelFormatEnum::RGB24).unwrap();
    let texture = surface.as_texture(&texture_builder).unwrap()
}
