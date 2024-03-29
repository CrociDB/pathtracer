extern crate sdl2; 

use std::time::{Duration, Instant};

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod tracer;

static DIMENSIONS:(u32, u32) = (800, 480);
 
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let (width, height) = DIMENSIONS;
 
    let window = video_subsystem.window("Pathtracer", width, height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        let mut screen:Option<Vec<u32>> = None;
        let mut time_elapsed = 0;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    let start_time = Instant::now();
                    screen = Some(tracer::trace(width, height));
                    time_elapsed = start_time.elapsed().as_secs();
                }
                _ => {}
            }
        }
        
        if let Some(pixels) = screen {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            for i in 0..height {
                for j in 0..width {
                    let pos = (i * width) + j;
                    let col = pixels[pos as usize];
                    let (r, g, b) = tracer::unpack_colors(col);

                    canvas.set_draw_color(Color::RGB(r, g, b));
                    let res = canvas.draw_point(sdl2::rect::Point::new(j as i32, i as i32));
                    if let Err(_) = res {
                        panic!("ERROR DRAWING POINT!");
                    }
                }
            }

            println!("Render time: {}", time_elapsed);
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
