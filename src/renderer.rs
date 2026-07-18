use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::{rect::Rect, video::Window};

use crate::context::{Context, GameState, Point};

/// 3-wide × 5-tall bitmap for digits 0–9 (row-major, top to bottom).
/// `true` means a lit pixel.
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

/// Spacing between digits in pixels.
const DIGIT_GAP: i32 = 2;

/// Pixel size of each "pixel" in a digit glyph.
const DIGIT_PX: u32 = 4;

/// Height reserved at the top of the window for the score HUD.
const SCORE_AREA_HEIGHT: i32 = 30;

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

    pub fn draw(&mut self, context: &Context) -> Result<(), String> {
        self.draw_background(context);

        let result = self
            .draw_walls(context)
            .and_then(|_| self.draw_player(context))
            .and_then(|_| self.draw_food(context))
            .and_then(|_| self.draw_score(context));

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

    fn draw_score(&mut self, context: &Context) -> Result<(), String> {
        let color = match context.state {
            GameState::Won => Color::RGB(0, 255, 100),
            GameState::Over => Color::RGB(255, 80, 80),
            _ => Color::RGB(180, 180, 180),
        };
        self.canvas.set_draw_color(color);

        let digits: Vec<u8> = if context.score == 0 {
            vec![0]
        } else {
            let mut n = context.score;
            let mut v = Vec::new();
            while n > 0 {
                v.push((n % 10) as u8);
                n /= 10;
            }
            v.reverse();
            v
        };

        // Center the score digits in the score area
        let digit_width = (3 * DIGIT_PX as i32) + DIGIT_GAP;
        let total_width = digits.len() as i32 * digit_width - DIGIT_GAP;
        let start_x = (self.screen_width as i32 - total_width) / 2;
        let start_y = (SCORE_AREA_HEIGHT - 5 * DIGIT_PX as i32) / 2;

        for (i, &digit) in digits.iter().enumerate() {
            let glyph = &DIGITS[digit as usize];
            for (row, cols) in glyph.iter().enumerate() {
                for (col, &on) in cols.iter().enumerate() {
                    if on {
                        let px = start_x + i as i32 * digit_width + col as i32 * DIGIT_PX as i32;
                        let py = start_y + row as i32 * DIGIT_PX as i32;
                        self.canvas
                            .fill_rect(Rect::new(px, py, DIGIT_PX, DIGIT_PX))?;
                    }
                }
            }
        }

        Ok(())
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
}
