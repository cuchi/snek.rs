extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};

mod context;
mod renderer;

use context::Context;
use renderer::Renderer;

pub fn main() -> Result<(), String> {
    let max_frame_duration = Duration::new(0, 1_000_000_000u32 / 60);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Snek", 800, 630)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut game_renderer = Renderer::new(window, 800)?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut game_context = Context::new();

    let mut accumulator = Duration::ZERO;
    let mut last_frame = Instant::now();

    'running: loop {
        let now = Instant::now();
        let delta = now.duration_since(last_frame);
        last_frame = now;
        accumulator += delta;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => game_context.toggle_pause(),
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => game_context.move_up(),
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => game_context.move_down(),
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => game_context.move_right(),
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => game_context.move_left(),
                _ => {}
            }
        }

        // Consume accumulated time in variable-length tick steps.
        // Tick duration decreases as score increases, so we re-read
        // it on every iteration of the loop.
        loop {
            let tick_duration = Duration::from_millis(game_context.tick_duration_ms());
            if accumulator < tick_duration {
                break;
            }
            game_context.next_tick();
            accumulator -= tick_duration;
        }

        game_renderer.draw(&game_context)?;

        // Sleep the remainder of the frame to avoid busy-waiting
        let elapsed = last_frame.elapsed();
        if elapsed < max_frame_duration {
            ::std::thread::sleep(max_frame_duration - elapsed);
        }
    }

    Ok(())
}
