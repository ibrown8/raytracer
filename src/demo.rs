use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};
use sdl2::surface::Surface;
use std::thread::sleep;
use raytrace::math::{Point, Vec3};
use raytrace::math::Color as Color3;
use raytrace::{Ray, Sphere, Hit};
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
    let mut texture = texture_builder.create_texture_streaming(PixelFormatEnum::RGB24, 640, 480).unwrap();
    canvas.set_draw_color(Color::RGB(128, 128, 128));
    canvas.clear();
    canvas.present();
    println!("Canvas presented");
    //Camera
    let aspect_ratio = 4.0 / 3.0;
    let focal_len = 1.0;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let camera_center = Point::new(0.0, 0.0, 0.0);
    //Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    let pixel_delta_u = viewport_u / 640.0;
    let pixel_delta_v = viewport_v / 480.0;
    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_len) - viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut paused : bool = false;
    let sphere = Sphere {
        loc : Point::new(0.0, 0.0, -1.0),
        radius : 0.5,
        color : Color3::new(1.0, 0.0, 0.0)
    };
    'running : loop {
        let loop_start = Instant::now();
        canvas.clear();
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
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..480u16 {
                for x in 0..640u16 {
                    let offset = (y as usize) * pitch + (x  as usize) * 3;
                    let pixel_center = pixel00_loc + (pixel_delta_v * (y as f32)) + (pixel_delta_u * (x as f32));
                    let ray_direction = pixel_center - camera_center;
                    let ray = Ray::new(&camera_center, &ray_direction);
                    let color = calculate_ray(&ray, &sphere).to_rgb();
                    buffer[offset + 0] = color.0;
                    buffer[offset + 1] = color.1;
                    buffer[offset + 2] = color.2;
                }
            }
        });
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
        let duration = now.elapsed();
        let loop_time = loop_start.elapsed();
        println!("It took {} ms to render the frame", duration.as_millis());
        println!("The loop took {} ms to run", loop_time.as_millis());
        //println!("Canvas presented");
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    //let surface = Surface::new(640, 480, PixelFormatEnum::RGB24).unwrap();
    //let texture = surface.as_texture(&texture_builder).unwrap()
}
