extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

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
 
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        let mut screen:Option<sdl2::surface::Surface> = None;


        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    screen = Some(trace(width, height));
                }
                _ => {}
            }
        }
        
        if let Some(surface) = screen {
            // Blit to screen
            println!("BLITTING!");
            screen = None;
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn trace(width:u32, height:u32) -> sdl2::surface::Surface<'static> {
    let pixels = tracer::trace(width, height);
    
    let masks = sdl2::pixels::PixelFormatEnum::RGBA32.into_masks().unwrap();
    let surface = sdl2::surface::Surface::from_pixelmasks(width, height, masks).unwrap();

    surface
}

