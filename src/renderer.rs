use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::{rect::Rect, video::Window};

use crate::context::{Context, GameState, Point};

// ---------------------------------------------------------------------------
// Pixel-font digit bitmaps (3 wide × 5 tall, row-major, top → bottom)
// ---------------------------------------------------------------------------

const DIGITS: [[[bool; 3]; 5]; 10] = [
    // 0
    [
        [true, true, true],
        [true, false, true],
        [true, false, true],
        [true, false, true],
        [true, true, true],
    ],
    // 1
    [
        [false, true, false],
        [true, true, false],
        [false, true, false],
        [false, true, false],
        [true, true, true],
    ],
    // 2
    [
        [true, true, true],
        [false, false, true],
        [true, true, true],
        [true, false, false],
        [true, true, true],
    ],
    // 3
    [
        [true, true, true],
        [false, false, true],
        [true, true, true],
        [false, false, true],
        [true, true, true],
    ],
    // 4
    [
        [true, false, true],
        [true, false, true],
        [true, true, true],
        [false, false, true],
        [false, false, true],
    ],
    // 5
    [
        [true, true, true],
        [true, false, false],
        [true, true, true],
        [false, false, true],
        [true, true, true],
    ],
    // 6
    [
        [true, true, true],
        [true, false, false],
        [true, true, true],
        [true, false, true],
        [true, true, true],
    ],
    // 7
    [
        [true, true, true],
        [false, false, true],
        [false, true, false],
        [true, false, false],
        [true, false, false],
    ],
    // 8
    [
        [true, true, true],
        [true, false, true],
        [true, true, true],
        [true, false, true],
        [true, true, true],
    ],
    // 9
    [
        [true, true, true],
        [true, false, true],
        [true, true, true],
        [false, false, true],
        [true, true, true],
    ],
];

const DIGIT_GAP: i32 = 2;
const DIGIT_PX: u32 = 4;
const SCORE_AREA_HEIGHT: i32 = 30;

/// Total pixel width of one digit glyph (glyph + gap).
fn digit_step() -> i32 {
    (3 * DIGIT_PX as i32) + DIGIT_GAP
}

// ---------------------------------------------------------------------------
// Renderer
// ---------------------------------------------------------------------------

pub struct Renderer {
    canvas: Canvas<Window>,
    screen_width: u32,
    dot_size: i32,
    dot_padding: i32,
    padded_dot_size: u32,
}

impl Renderer {
    pub fn new(window: Window, screen_width: u32) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        let dot_size: i32 = 20;
        let dot_padding: i32 = 2;
        let padded_dot_size: u32 = (dot_size - 2 * dot_padding) as u32;

        Ok(Renderer {
            canvas,
            screen_width,
            dot_size,
            dot_padding,
            padded_dot_size,
        })
    }

    // -- top-level draw ---------------------------------------------------

    pub fn draw(&mut self, context: &Context) -> Result<(), String> {
        self.draw_background(context);

        let mut result = self
            .draw_walls(context)
            .and_then(|_| self.draw_player(context))
            .and_then(|_| self.draw_food(context))
            .and_then(|_| self.draw_score(context));

        // High-scores overlay during game-over / win
        if matches!(context.state, GameState::Over | GameState::Won) {
            result = result.and_then(|_| self.draw_high_scores(context));
        }

        self.canvas.present();
        result
    }

    fn draw_background(&mut self, context: &Context) {
        let color = match context.state {
            GameState::Playing => Color::RGB(0, 0, 0),
            GameState::Paused => Color::RGB(30, 30, 30),
            GameState::Over => Color::RGB(60, 0, 0),
            GameState::Won => Color::RGB(0, 40, 0),
        };
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    // -- game elements ----------------------------------------------------

    fn draw_walls(&mut self, context: &Context) -> Result<(), String> {
        let Point(x_size, y_size) = context.board_size;
        self.canvas.set_draw_color(Color::WHITE);

        for x in 0..x_size {
            self.draw_dot(Point(x, 0))?;
            self.draw_dot(Point(x, y_size - 1))?;
        }

        for y in 1..(y_size - 1) {
            self.draw_dot(Point(0, y))?;
            self.draw_dot(Point(x_size - 1, y))?;
        }

        Ok(())
    }

    fn draw_player(&mut self, context: &Context) -> Result<(), String> {
        self.canvas.set_draw_color(Color::GREEN);
        for point in &context.player_position {
            self.draw_dot(*point)?;
        }
        Ok(())
    }

    fn draw_food(&mut self, context: &Context) -> Result<(), String> {
        match context.food {
            None => Ok(()),
            Some(food) => {
                self.canvas.set_draw_color(Color::RED);
                self.draw_dot(food)?;
                Ok(())
            }
        }
    }

    fn draw_dot(&mut self, point: Point) -> Result<(), String> {
        let Point(x, y) = point;
        self.canvas.fill_rect(Rect::new(
            x * self.dot_size + self.dot_padding,
            y * self.dot_size + self.dot_padding + SCORE_AREA_HEIGHT,
            self.padded_dot_size,
            self.padded_dot_size,
        ))?;
        Ok(())
    }

    // -- score HUD --------------------------------------------------------

    fn draw_score(&mut self, context: &Context) -> Result<(), String> {
        let color = match context.state {
            GameState::Won => Color::RGB(0, 255, 100),
            GameState::Over => Color::RGB(255, 80, 80),
            _ => Color::RGB(180, 180, 180),
        };

        let n = context.score;
        let w = number_pixel_width(n);
        let x = (self.screen_width as i32 - w) / 2;
        let y = (SCORE_AREA_HEIGHT - 5 * DIGIT_PX as i32) / 2;

        self.draw_number(n, x, y, color)
    }

    // -- high-scores overlay ----------------------------------------------

    fn draw_high_scores(&mut self, context: &Context) -> Result<(), String> {
        if context.high_scores.is_empty() {
            return Ok(());
        }

        let line_height = 5 * DIGIT_PX as i32 + 4;
        let total_height = context.high_scores.len() as i32 * line_height;

        // Center the list vertically on the board
        let board_px_height = context.board_size.1 * self.dot_size;
        let board_center_y = SCORE_AREA_HEIGHT + board_px_height / 2;
        let start_y = board_center_y - total_height / 2;

        // Find the widest line so we can center horizontally
        let mut max_line_w = 0;
        for (rank, &score) in context.high_scores.iter().enumerate() {
            // "R  SCORE"  (rank-digit, gap, score-digits)
            let w = number_pixel_width(rank as u32 + 1) + 12 + number_pixel_width(score);
            max_line_w = max_line_w.max(w);
        }
        let start_x = (self.screen_width as i32 - max_line_w) / 2;

        let color = match context.state {
            GameState::Won => Color::RGB(0, 220, 120),
            _ => Color::RGB(220, 220, 220),
        };

        for (i, &score) in context.high_scores.iter().enumerate() {
            let rank = i as u32 + 1;
            let y = start_y + i as i32 * line_height;

            // Render rank digit
            self.draw_number(rank, start_x, y, color)?;

            // Render gap (just offset)
            let score_x = start_x + number_pixel_width(rank) + 12;
            self.draw_number(score, score_x, y, color)?;
        }

        Ok(())
    }

    // -- pixel-font helpers -----------------------------------------------

    /// Render a single digit glyph at pixel position (x, y).
    fn draw_digit(&mut self, digit: u8, x: i32, y: i32) -> Result<(), String> {
        let glyph = &DIGITS[digit as usize];
        for (row, cols) in glyph.iter().enumerate() {
            for (col, &on) in cols.iter().enumerate() {
                if on {
                    let px = x + col as i32 * DIGIT_PX as i32;
                    let py = y + row as i32 * DIGIT_PX as i32;
                    self.canvas
                        .fill_rect(Rect::new(px, py, DIGIT_PX, DIGIT_PX))?;
                }
            }
        }
        Ok(())
    }

    /// Render a full number (≥ 0) at pixel position (x, y) in the given colour.
    fn draw_number(&mut self, n: u32, x: i32, y: i32, color: Color) -> Result<(), String> {
        self.canvas.set_draw_color(color);

        let digits = digits_of(n);
        let step = digit_step();

        for (i, &d) in digits.iter().enumerate() {
            self.draw_digit(d, x + i as i32 * step, y)?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Free helpers
// ---------------------------------------------------------------------------

/// Split a number into its decimal digits (most-significant first).
/// `0` → `[0]`.
fn digits_of(mut n: u32) -> Vec<u8> {
    if n == 0 {
        return vec![0];
    }
    let mut v = Vec::new();
    while n > 0 {
        v.push((n % 10) as u8);
        n /= 10;
    }
    v.reverse();
    v
}

/// Pixel width of a number when rendered with the pixel font.
fn number_pixel_width(n: u32) -> i32 {
    let count = if n == 0 { 1 } else { (n.ilog10() + 1) as i32 };
    count * digit_step() - DIGIT_GAP
}
