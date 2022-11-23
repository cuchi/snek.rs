extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod context;
mod renderer;

use context::Context;
use renderer::Renderer;

const KEY_W: Option<Keycode> = Some(Keycode::W);
const KEY_A: Option<Keycode> = Some(Keycode::A);
const KEY_S: Option<Keycode> = Some(Keycode::S);
const KEY_D: Option<Keycode> = Some(Keycode::D);
const KEY_ESC: Option<Keycode> = Some(Keycode::Escape);

pub fn main() -> Result<(), String> {
    let frame_duration = Duration::new(0, 1_000_000_000u32 / 30);
    let frames_per_tick = 6; // 5 ticks per second

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Snek", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut game_renderer = Renderer::new(window)?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut game_context = Context::new();
    let mut frame_counter: u64 = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: KEY_ESC, ..
                } => game_context.toggle_pause(),
                Event::KeyDown { keycode: KEY_W, .. } => game_context.move_up(),
                Event::KeyDown { keycode: KEY_S, .. } => game_context.move_down(),
                Event::KeyDown { keycode: KEY_D, .. } => game_context.move_right(),
                Event::KeyDown { keycode: KEY_A, .. } => game_context.move_left(),
                _ => {}
            }
        }

        ::std::thread::sleep(frame_duration);
        if frame_counter % frames_per_tick == 0 {
            game_context.next_tick();
        }
        game_renderer.draw(&game_context)?;
        frame_counter += 1;
    }

    Ok(())
}
