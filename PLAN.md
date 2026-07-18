# Plan

This document tracks findings from the code review and the roadmap for future features.

---

## Code Review — Completed Fixes

| # | File | Issue | Fix |
|---|------|-------|-----|
| 1 | `src/context.rs` | `move_player` reversed the `Vec` twice just to push to the front | Use `self.player_position.insert(0, ...)` |
| 2 | `src/context.rs` | `is_game_over` took `&mut self` but never mutated | Changed to `&self` |
| 3 | `src/context.rs` | `iter().any()` used where `contains()` works (×2) | Replaced with `.contains()` |
| 4 | `src/renderer.rs` | `padded_dot_size` hardcoded to `18` | Derived from `dot_size - 2 * dot_padding` |
| 5 | `src/main.rs` | `KEY_*` constants added unnecessary indirection | Inlined `Some(Keycode::*)` directly in match arms |
| 6 | `src/main.rs` | `frame_counter % frames_per_tick == 0` (clippy) | Changed to `frame_counter.is_multiple_of(frames_per_tick)` |
| 7 | `Cargo.toml` | Boilerplate comment retained | Removed |

## Code Review — Open Issues (Resolved)

| # | File | Issue | Fix |
|---|------|-------|-----|
| 1 | `src/context.rs` | `spawn_food` loops forever if the board fills up | Capped attempts at board area; sets `GameState::Won` if no empty cells remain |
| 2 | `src/renderer.rs` | `draw_walls` draws corner tiles twice | Vertical wall loop now uses `1..(y_size - 1)` to skip corners |
| 3 | `src/renderer.rs` | `?` in draw loops bails early without `canvas.present()` | `draw()` collects all sub-draw results and calls `canvas.present()` unconditionally before propagating the error |
| 4 | `src/main.rs` | Fixed `thread::sleep` can drift | Replaced with a delta-time accumulator (`Instant`-based) with fixed tick steps |
| 5 | — | Zero tests | Added 30 unit tests covering: direction reversals, wall/self collision, food spawning, score, game state transitions, pause/restart, board-full win condition, speed curve, tick event returns |
| 6 | — | No score tracking or display | Added `score: u32` field to `Context`; on-canvas pixel-font rendering with colour-coded states |
| 7 | `.cargo/config.toml` | Hardcoded Apple Silicon path breaks other platforms | Added comments documenting platform setup, including Intel Mac fallback |

---

## Feature Roadmap

### P1 — High Priority

- [x] **Visual score rendering on canvas**
  - 3×5 pixel-font digits drawn with `fill_rect`, centred in a 30px HUD bar above the board.
  - Score colour changes on game-over (red) and win (green).
  - No extra dependencies (no SDL2_ttf needed).

- [x] **Speed increases as snake grows**
  - `Context::tick_duration_ms()` decreases by 8 ms per food eaten, from 200 ms down to a 60 ms floor.
  - Main loop re-reads the tick duration dynamically every iteration.
  - 4 new tests verify the speed curve and floor behaviour.

### P2 — Medium Priority

- [x] **High-score persistence**
  - New `src/high_scores.rs` module: loads/saves top-5 scores to `~/.snek_highscores`.
  - Scores are persisted on game-over and when the board fills (win).
  - Top-5 list rendered as a pixel-font overlay centred on the board during game-over/win screens.
  - Refactored pixel-font digit drawing into reusable helpers (`draw_digit`, `draw_number`, `digits_of`, `number_pixel_width`).

- [x] **Sound effects**
  - New `src/audio.rs` module using SDL2_mixer.
  - WAV data generated programmatically (no asset files): rising chirp (eat), descending buzz (death), ascending fanfare (win).
  - `Context::next_tick()` now returns a `TickEvent` enum (`None`/`Ate`/`Died`/`Won`) so `main.rs` can react and play the appropriate sound.
  - 2 new tests verify tick event return values.
  - Requires `sdl2_mixer` system library (`brew install sdl2_mixer` on macOS).

### P3 — Nice-to-Have

- [ ] **Smooth rendering / interpolation**
  - Interpolate snake position between ticks for fluid movement.
  - _Effort: Medium_

- [ ] **Cross-platform CI**
  - GitHub Actions workflow building on Linux, macOS, Windows.
  - Run tests and clippy on PRs.
  - _Effort: Medium_

- [ ] **Configurable controls / board size**
  - Load keybindings and board dimensions from a config file or CLI args.
  - _Effort: Small-Medium_
