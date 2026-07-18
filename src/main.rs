extern crate sdl2;

use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};

mod audio;
mod context;
mod high_scores;
mod renderer;

use context::{Context, TickEvent};
use renderer::Renderer;

/// A processed input event we know how to handle.
enum Input {
    Quit,
    Key(Keycode),
}

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

    let audio = audio::Audio::init()?;

    // Use the raw event pump handle — we poll events ourselves to avoid
    // panics on unknown event types that sdl2 0.35 can't decode.
    let mut event_pump = sdl_context.event_pump()?;
    // Drain any queued events from audio init.
    drain_raw_events(&mut event_pump);

    let mut game_context = Context::new();

    let mut accumulator = Duration::ZERO;
    let mut last_frame = Instant::now();

    'running: loop {
        let now = Instant::now();
        let delta = now.duration_since(last_frame);
        last_frame = now;
        accumulator += delta;

        for input in poll_inputs(&mut event_pump) {
            match input {
                Input::Quit => break 'running,
                Input::Key(Keycode::Escape) => game_context.toggle_pause(),
                Input::Key(Keycode::W) => game_context.move_up(),
                Input::Key(Keycode::S) => game_context.move_down(),
                Input::Key(Keycode::D) => game_context.move_right(),
                Input::Key(Keycode::A) => game_context.move_left(),
                _ => {}
            }
        }

        loop {
            let tick_duration = Duration::from_millis(game_context.tick_duration_ms());
            if accumulator < tick_duration {
                break;
            }
            let tick_event = game_context.next_tick();
            match tick_event {
                TickEvent::Ate => audio.play_eat(),
                TickEvent::Died => audio.play_death(),
                TickEvent::Won => audio.play_win(),
                TickEvent::None => {}
            }
            accumulator -= tick_duration;
        }

        game_renderer.draw(&game_context)?;

        let elapsed = last_frame.elapsed();
        if elapsed < max_frame_duration {
            ::std::thread::sleep(max_frame_duration - elapsed);
        }
    }

    Ok(())
}

/// Poll raw SDL events, returning only ones we recognise.
///
/// Uses `SDL_PollEvent` directly rather than the sdl2 crate's `Event`
/// conversion, which panics on unknown event types (e.g. events emitted
/// by SDL2-compat / SDL3 back-ends).
fn poll_inputs(_pump: &mut sdl2::EventPump) -> Vec<Input> {
    use sdl2::sys::{SDL_Event, SDL_EventType, SDL_PollEvent};

    let mut inputs = Vec::new();
    let mut raw: SDL_Event = unsafe { std::mem::zeroed() };

    while unsafe { SDL_PollEvent(&mut raw) } != 0 {
        // SAFETY: raw is a valid SDL_Event filled by SDL_PollEvent.
        let event_type = unsafe { raw.type_ };

        if event_type == SDL_EventType::SDL_QUIT as u32 {
            inputs.push(Input::Quit);
        } else if event_type == SDL_EventType::SDL_KEYDOWN as u32 {
            // SAFETY: raw.key is valid for KEYDOWN events.
            let sym = unsafe { raw.key.keysym.sym };
            if let Some(kc) = Keycode::from_i32(sym) {
                inputs.push(Input::Key(kc));
            }
        }
        // All other event types are silently discarded.
    }

    inputs
}

/// Drain and discard all pending raw events (used after initialisation).
fn drain_raw_events(_pump: &mut sdl2::EventPump) {
    use sdl2::sys::{SDL_Event, SDL_PollEvent};
    let mut raw: SDL_Event = unsafe { std::mem::zeroed() };
    while unsafe { SDL_PollEvent(&mut raw) } != 0 {
        // discard
    }
}
