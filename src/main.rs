extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod context;
mod renderer;

use context::Context;
use renderer::Renderer;

pub fn main() -> Result<(), String> {
    // 30 FPS rendering, game logic ticks every 6th frame = 5 ticks/sec
    let frame_duration = Duration::new(0, 1_000_000_000u32 / 30);
    let frames_per_tick: u64 = 6;

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

        ::std::thread::sleep(frame_duration);
        if frame_counter.is_multiple_of(frames_per_tick) {
            game_context.next_tick();
        }
        game_renderer.draw(&game_context)?;
        frame_counter += 1;
    }

    Ok(())
}
