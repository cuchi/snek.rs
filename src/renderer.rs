use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::{rect::Rect, video::Window};

use crate::context::{Context, GameState, Point};

pub struct Renderer {
    canvas: Canvas<Window>,
    dot_size: i32,
    dot_padding: i32,
    padded_dot_size: u32,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        Ok(Renderer {
            canvas,
            dot_size: 20,
            dot_padding: 2,
            padded_dot_size: 18,
        })
    }

    pub fn draw(&mut self, context: &Context) -> Result<(), String> {
        self.draw_background(context);
        self.draw_walls(context)?;
        self.draw_player(context)?;
        self.draw_food(context)?;
        self.canvas.present();

        Ok(())
    }

    fn draw_background(&mut self, context: &Context) {
        let color = match context.state {
            GameState::Playing => Color::RGB(0, 0, 0),
            GameState::Paused => Color::RGB(30, 30, 30),
            GameState::Over => Color::RGB(60, 0, 0),
        };
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    fn draw_walls(&mut self, context: &Context) -> Result<(), String> {
        let Point(x_size, y_size) = context.board_size;
        self.canvas.set_draw_color(Color::WHITE);
        for i in 0..x_size {
            self.draw_dot(Point(i, 0))?;
            self.draw_dot(Point(i, y_size - 1))?;
        }

        for i in 0..y_size {
            self.draw_dot(Point(0, i))?;
            self.draw_dot(Point(x_size - 1, i))?;
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
            y * self.dot_size + self.dot_padding,
            self.padded_dot_size,
            self.padded_dot_size,
        ))?;

        Ok(())
    }
}
