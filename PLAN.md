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
| 3 | `src/renderer.rs` | `?` in draw loops bails early without `canvas.present()` | `draw()` collects all sub-draw results and calls `canvas.present()` unconditionally |
| 4 | `src/main.rs` | Fixed `thread::sleep` can drift | Replaced with a delta-time accumulator (`Instant`-based) |
| 5 | — | Zero tests | 30 unit tests: collision, state transitions, direction reversals, scoring, speed curve, tick events |
| 6 | — | No score tracking or display | On-canvas pixel-font score HUD with colour-coded states |
| 7 | `.cargo/config.toml` | Hardcoded Apple Silicon path breaks other platforms | Documented platform setup with Intel Mac fallback |

---

## Feature Roadmap

### P1 — High Priority

- [x] **Visual score rendering on canvas**
- [x] **Speed increases as snake grows**

### P2 — Medium Priority

- [x] **High-score persistence**
- [x] **Sound effects** (SDL2_mixer, programmatic WAV generation)

### P3 — Nice-to-Have

- [x] **Cross-platform CI**
  - `.github/workflows/ci.yml` — runs on push/PR to `master`.
  - Matrix: `ubuntu-latest`, `macos-latest`, `windows-latest`.
  - Installs SDL2 + SDL2_mixer per-platform (apt, brew, pre-built VC zips).
  - Runs `cargo build`, `cargo test`, and `cargo clippy -- -D warnings`.
  - `fail-fast: false` so one platform failing doesn't cancel the others.

- [ ] **Configurable controls / board size**
  - Load keybindings and board dimensions from a config file or CLI args.
  - _Effort: Small-Medium_
