//! Minimal sound effects via SDL2_mixer.
//!
//! WAV data is generated programmatically (no external asset files) and
//! written to the system temp directory on first load.

use sdl2::mixer::{Channel, Chunk};
use std::fs;
use std::io::Write;
use std::path::Path;

const SAMPLE_RATE: u32 = 22050;

/// Write a minimal WAV file (PCM, mono, 16-bit, 22050 Hz) to `path`.
fn write_wav(path: &Path, samples: &[i16]) {
    let data_size = (samples.len() * 2) as u32;
    let file_size = 36 + data_size;

    let mut f = fs::File::create(path).expect("failed to create temp WAV");
    // RIFF header
    f.write_all(b"RIFF").unwrap();
    f.write_all(&file_size.to_le_bytes()).unwrap();
    f.write_all(b"WAVE").unwrap();
    // fmt  chunk
    f.write_all(b"fmt ").unwrap();
    f.write_all(&16u32.to_le_bytes()).unwrap(); // chunk size
    f.write_all(&1u16.to_le_bytes()).unwrap(); // PCM
    f.write_all(&1u16.to_le_bytes()).unwrap(); // mono
    f.write_all(&SAMPLE_RATE.to_le_bytes()).unwrap();
    f.write_all(&(SAMPLE_RATE * 2).to_le_bytes()).unwrap(); // byte rate
    f.write_all(&2u16.to_le_bytes()).unwrap(); // block align
    f.write_all(&16u16.to_le_bytes()).unwrap(); // bits per sample
                                                // data chunk
    f.write_all(b"data").unwrap();
    f.write_all(&data_size.to_le_bytes()).unwrap();
    // PCM samples
    for &s in samples {
        f.write_all(&s.to_le_bytes()).unwrap();
    }
}

/// Generate a rising "chirp" for eating food (~80 ms).
fn eat_samples() -> Vec<i16> {
    let duration = 0.08;
    let len = (SAMPLE_RATE as f64 * duration) as usize;
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        let t = i as f64 / SAMPLE_RATE as f64;
        let freq = 400.0 + 400.0 * (t / duration); // 400 → 800 Hz
        let amp = 0.3 * (1.0 - t / duration); // fade out
        let sample = amp * (2.0 * std::f64::consts::PI * freq * t).sin();
        out.push((sample * 32767.0) as i16);
    }
    out
}

/// Generate a descending "buzz" for dying (~250 ms).
fn death_samples() -> Vec<i16> {
    let duration = 0.25;
    let len = (SAMPLE_RATE as f64 * duration) as usize;
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        let t = i as f64 / SAMPLE_RATE as f64;
        let freq = 300.0 - 220.0 * (t / duration); // 300 → 80 Hz
        let amp = 0.3 * (1.0 - t / duration); // fade out
        let sample = amp * (2.0 * std::f64::consts::PI * freq * t).sin();
        out.push((sample * 32767.0) as i16);
    }
    out
}

/// Generate a short "fanfare" for winning (~400 ms).
fn win_samples() -> Vec<i16> {
    let duration = 0.4;
    let len = (SAMPLE_RATE as f64 * duration) as usize;
    let mut out = Vec::with_capacity(len);
    let notes = [523.0, 659.0, 784.0, 1047.0]; // C5 E5 G5 C6
    let note_len = len / notes.len();
    for &freq in &notes {
        for i in 0..note_len {
            let t = i as f64 / SAMPLE_RATE as f64;
            let amp = 0.25;
            let sample = amp * (2.0 * std::f64::consts::PI * freq * t).sin();
            out.push((sample * 32767.0) as i16);
        }
    }
    out
}

/// Holds pre-loaded sound chunks.
pub struct Audio {
    eat: Chunk,
    death: Chunk,
    win: Chunk,
}

impl Audio {
    /// Initialise SDL2_mixer and load sounds (generating WAVs if needed).
    pub fn init() -> Result<Audio, String> {
        sdl2::mixer::open_audio(SAMPLE_RATE as i32, sdl2::mixer::AUDIO_S16LSB, 1, 512)?;
        sdl2::mixer::allocate_channels(4);

        let tmp = std::env::temp_dir().join("snek_audio");
        fs::create_dir_all(&tmp).map_err(|e| e.to_string())?;

        let eat = Self::load(&tmp, "eat.wav", &eat_samples())?;
        let death = Self::load(&tmp, "death.wav", &death_samples())?;
        let win = Self::load(&tmp, "win.wav", &win_samples())?;

        Ok(Audio { eat, death, win })
    }

    fn load(dir: &Path, name: &str, samples: &[i16]) -> Result<Chunk, String> {
        let path = dir.join(name);
        write_wav(&path, samples);
        Chunk::from_file(&path).map_err(|e| e.to_string())
    }

    pub fn play_eat(&self) {
        let _ = Channel::all().play(&self.eat, 0);
    }

    pub fn play_death(&self) {
        let _ = Channel::all().play(&self.death, 0);
    }

    pub fn play_win(&self) {
        let _ = Channel::all().play(&self.win, 0);
    }
}
