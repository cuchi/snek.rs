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
| 5 | — | Zero tests | Added 24 unit tests covering: direction reversals, wall/self collision, food spawning, score, game state transitions, pause/restart, board-full win condition |
| 6 | — | No score tracking or display | Added `score: u32` field to `Context`; displayed in window title bar via `canvas.window_mut().set_title()` |
| 7 | `.cargo/config.toml` | Hardcoded Apple Silicon path breaks other platforms | Added comments documenting platform setup, including Intel Mac fallback |

---

## Feature Roadmap

### P1 — High Priority

- [ ] **Visual score rendering on canvas**
  - Replace window-title score with on-screen text (SDL2_ttf or bitmap digits).
  - _Effort: Small_

- [ ] **Speed increases as snake grows**
  - Reduce tick interval after each food eaten.
  - _Effort: Small_

### P2 — Medium Priority

- [ ] **High-score persistence**
  - Write top scores to a local file (`~/.snek_highscores`).
  - Display top 5 scores on game over.
  - _Effort: Medium_

- [ ] **Sound effects**
  - Add SDL2_mixer dependency.
  - Eat-food sound, death sound.
  - _Effort: Medium_

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
