use std::ops::Add;

use rand::{rngs::ThreadRng, Rng};

pub enum GameState {
    Playing,
    Paused,
    Over,
}

#[derive(Copy, Clone, PartialEq)]
pub enum PlayerDirection {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Point(pub i32, pub i32);

pub struct Context {
    pub state: GameState,
    pub player_position: Vec<Point>,
    pub player_direction: PlayerDirection,
    pub food: Option<Point>,
    pub board_size: Point,
    last_tick_direction: PlayerDirection,
    rng: ThreadRng,
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Context {
    pub fn new() -> Context {
        Context {
            state: GameState::Paused,
            player_position: vec![Point(20, 15), Point(19, 15), Point(18, 15)],
            player_direction: PlayerDirection::Right,
            last_tick_direction: PlayerDirection::Right,
            food: None,
            board_size: Point(40, 30),
            rng: rand::thread_rng(),
        }
    }

    pub fn next_tick(&mut self) {
        if self.food.is_none() {
            self.spawn_food();
        }
        if let GameState::Over | GameState::Paused = self.state {
            return;
        }
        let head_position = self.player_position.first().unwrap();
        let next_head_position = match self.player_direction {
            PlayerDirection::Up => *head_position + Point(0, -1),
            PlayerDirection::Down => *head_position + Point(0, 1),
            PlayerDirection::Right => *head_position + Point(1, 0),
            PlayerDirection::Left => *head_position + Point(-1, 0),
        };

        if self.is_game_over(next_head_position) {
            self.state = GameState::Over;
            return;
        }

        if matches!(self.food, Some(food) if food == next_head_position) {
            self.move_player(next_head_position, true);
            self.food = None;
            return;
        }

        self.move_player(next_head_position, false);
    }

    fn spawn_food(&mut self) {
        let Point(size_x, size_y) = self.board_size;

        loop {
            let food = Point(
                self.rng.gen_range(1..(size_x - 1)),
                self.rng.gen_range(1..(size_y - 1)),
            );
            if !self.player_position.iter().any(|dot| *dot == food) {
                self.food = Some(food);
                return;
            }
        }
    }

    fn move_player(&mut self, next_head_position: Point, grow: bool) {
        if !grow {
            self.player_position.pop();
        }
        self.player_position.reverse();
        self.player_position.push(next_head_position);
        self.player_position.reverse();
        self.last_tick_direction = self.player_direction;
    }

    pub fn move_up(&mut self) {
        if self.last_tick_direction == PlayerDirection::Down {
            return;
        }
        self.player_direction = PlayerDirection::Up;
    }

    pub fn move_down(&mut self) {
        if self.last_tick_direction == PlayerDirection::Up {
            return;
        }
        self.player_direction = PlayerDirection::Down;
    }

    pub fn move_right(&mut self) {
        if self.last_tick_direction == PlayerDirection::Left {
            return;
        }
        self.player_direction = PlayerDirection::Right;
    }

    pub fn move_left(&mut self) {
        if self.last_tick_direction == PlayerDirection::Right {
            return;
        }
        self.player_direction = PlayerDirection::Left;
    }

    fn is_game_over(&mut self, next_head_position: Point) -> bool {
        let Point(x, y) = next_head_position;
        let Point(x_size, y_size) = self.board_size;

        x == 0
            || y == 0
            || x == x_size - 1
            || y == y_size - 1
            || self
                .player_position
                .iter()
                .any(|point| *point == next_head_position)
    }

    pub fn toggle_pause(&mut self) {
        self.state = match self.state {
            GameState::Playing => GameState::Paused,
            GameState::Paused => GameState::Playing,
            GameState::Over => {
                self.player_position = vec![Point(20, 15), Point(19, 15), Point(18, 15)];
                self.player_direction = PlayerDirection::Right;
                self.last_tick_direction = PlayerDirection::Right;
                GameState::Playing
            }
        }
    }
}
