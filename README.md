# Snek.rs

A minimal Snake game written in Rust using SDL2.

![Screenshot](./screenshot.png)

[![CI](https://github.com/cuchi/snek.rs/actions/workflows/ci.yml/badge.svg)](https://github.com/cuchi/snek.rs/actions/workflows/ci.yml)

## Gameplay

- Control a snake with **WASD** keys.
- Eat red food pellets to grow longer and increase your score.
- The game speeds up as your score increases.
- Avoid hitting the walls or your own tail — that's game over.
- Fill the entire board to win!
- Top 5 high scores are persisted to disk and displayed on the game-over screen.
- Press **Escape** to pause, unpause, or restart after game over.

### Keybindings

| Key        | Action                        |
|------------|-------------------------------|
| **W**      | Move up                       |
| **A**      | Move left                     |
| **S**      | Move down                     |
| **D**      | Move right                    |
| **Escape** | Pause / Unpause / Restart     |

## Requirements

- **Rust** — edition 2021 (stable).
- **SDL2** and **SDL2_mixer** development libraries.

### Platform-specific setup

**macOS (Apple Silicon):**

```sh
brew install sdl2 sdl2_mixer
```

The `.cargo/config.toml` already points to Homebrew's library path. For Intel Macs, adjust the config or use `/usr/local/lib`.

**Linux (Debian/Ubuntu):**

```sh
sudo apt-get install libsdl2-dev libsdl2-mixer-dev
```

Other distributions: install the equivalent `sdl2` and `sdl2_mixer` development packages.

**Windows:**

Download the [SDL2](https://github.com/libsdl-org/SDL/releases) and [SDL2_mixer](https://github.com/libsdl-org/SDL_mixer/releases) VC development zips and set the `SDL2_LIB_DIR` environment variable. See the [rust-sdl2 Windows guide](https://github.com/Rust-SDL2/rust-sdl2#windows-msvc) for details.

## Building & Running

```sh
git clone https://github.com/your-username/snek.rs.git
cd snek.rs
cargo run --release
```

For development:

```sh
cargo run
```

Run the test suite:

```sh
cargo test
```

## Architecture

| File                                  | Responsibility                                                        |
|---------------------------------------|-----------------------------------------------------------------------|
| [`src/main.rs`](./src/main.rs)         | SDL2 init, event loop, delta-time tick accumulator, input & audio     |
| [`src/context.rs`](./src/context.rs)   | Game state: snake position/direction, food spawning, collision, score |
| [`src/renderer.rs`](./src/renderer.rs) | SDL2 drawing: background, walls, snake, food, pixel-font score HUD    |
| [`src/audio.rs`](./src/audio.rs)       | SDL2_mixer wrapper — generates WAV sounds programmatically            |
| [`src/high_scores.rs`](./src/high_scores.rs) | Loads/saves top-5 high scores to `~/.snek_highscores`           |

The game renders at up to 60 FPS with game logic ticking at a variable rate (200 ms → 60 ms as the score increases). A delta-time accumulator ensures consistent timing regardless of frame rate. Movement is intentionally discrete (grid-based, cell-by-cell) — no interpolation.

## Features

- On-screen pixel-font score display with colour-coded game states
- Progressive difficulty — game speeds up as you eat
- Top-5 high scores persisted in `~/.snek_highscores`
- Sound effects (eat chirp, death buzz, win fanfare) — no external audio files needed
- 30 unit tests covering game logic
- Cross-platform CI (Linux, macOS, Windows)

## Known Limitations

- Board size and keybindings are not configurable.
- SDL2_mixer must be installed as a system library (not vendored).

## License

MIT
