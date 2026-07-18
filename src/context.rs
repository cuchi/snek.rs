use std::ops::Add;

use rand::{rngs::ThreadRng, Rng};

use crate::high_scores;

pub enum GameState {
    Playing,
    Paused,
    Over,
    Won,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PlayerDirection {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point(pub i32, pub i32);

pub struct Context {
    pub state: GameState,
    pub player_position: Vec<Point>,
    pub player_direction: PlayerDirection,
    pub food: Option<Point>,
    pub board_size: Point,
    pub score: u32,
    pub high_scores: Vec<u32>,
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
            score: 0,
            high_scores: high_scores::load(),
            rng: rand::thread_rng(),
        }
    }

    /// Tick interval in milliseconds. Starts at 200 ms and decreases
    /// by 8 ms per food eaten, with a floor of 60 ms.
    pub fn tick_duration_ms(&self) -> u64 {
        let reduction = (self.score as u64).saturating_mul(8);
        200u64.saturating_sub(reduction).max(60)
    }

    pub fn next_tick(&mut self) {
        if let GameState::Over | GameState::Paused | GameState::Won = self.state {
            return;
        }
        if self.food.is_none() {
            self.spawn_food();
        }
        // If spawn_food failed (board full), state is now Won — persist & exit
        if let GameState::Won = self.state {
            high_scores::maybe_insert(self.score, &mut self.high_scores);
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
            high_scores::maybe_insert(self.score, &mut self.high_scores);
            return;
        }

        if matches!(self.food, Some(food) if food == next_head_position) {
            self.move_player(next_head_position, true);
            self.food = None;
            self.score += 1;
            return;
        }

        self.move_player(next_head_position, false);
    }

    fn spawn_food(&mut self) {
        let Point(size_x, size_y) = self.board_size;
        let max_attempts = ((size_x - 2) * (size_y - 2)) as usize;

        for _ in 0..max_attempts {
            let food = Point(
                self.rng.gen_range(1..(size_x - 1)),
                self.rng.gen_range(1..(size_y - 1)),
            );
            if !self.player_position.contains(&food) {
                self.food = Some(food);
                return;
            }
        }

        // Board is full — player wins
        self.state = GameState::Won;
    }

    fn move_player(&mut self, next_head_position: Point, grow: bool) {
        if !grow {
            self.player_position.pop();
        }
        self.player_position.insert(0, next_head_position);
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

    fn is_game_over(&self, next_head_position: Point) -> bool {
        let Point(x, y) = next_head_position;
        let Point(x_size, y_size) = self.board_size;

        x == 0
            || y == 0
            || x == x_size - 1
            || y == y_size - 1
            || self.player_position.contains(&next_head_position)
    }

    pub fn toggle_pause(&mut self) {
        self.state = match self.state {
            GameState::Playing => GameState::Paused,
            GameState::Paused => GameState::Playing,
            GameState::Over | GameState::Won => {
                self.player_position = vec![Point(20, 15), Point(19, 15), Point(18, 15)];
                self.player_direction = PlayerDirection::Right;
                self.last_tick_direction = PlayerDirection::Right;
                self.food = None;
                self.score = 0;
                GameState::Playing
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_context() -> Context {
        Context::new()
    }

    #[test]
    fn starts_paused() {
        let ctx = new_context();
        assert!(matches!(ctx.state, GameState::Paused));
    }

    #[test]
    fn starts_with_score_zero() {
        let ctx = new_context();
        assert_eq!(ctx.score, 0);
    }

    #[test]
    fn starts_with_three_segments() {
        let ctx = new_context();
        assert_eq!(ctx.player_position.len(), 3);
    }

    #[test]
    fn unpause_on_escape() {
        let mut ctx = new_context();
        ctx.toggle_pause();
        assert!(matches!(ctx.state, GameState::Playing));
    }

    #[test]
    fn pause_on_escape_when_playing() {
        let mut ctx = new_context();
        ctx.toggle_pause(); // unpause
        ctx.toggle_pause(); // pause again
        assert!(matches!(ctx.state, GameState::Paused));
    }

    #[test]
    fn restart_on_escape_when_game_over() {
        let mut ctx = new_context();
        ctx.toggle_pause(); // start playing
        ctx.state = GameState::Over;
        ctx.score = 5;
        ctx.toggle_pause(); // restart
        assert!(matches!(ctx.state, GameState::Playing));
        assert_eq!(ctx.score, 0);
        assert_eq!(ctx.player_position.len(), 3);
    }

    #[test]
    fn restart_on_escape_when_won() {
        let mut ctx = new_context();
        ctx.toggle_pause(); // start playing
        ctx.state = GameState::Won;
        ctx.score = 42;
        ctx.toggle_pause(); // restart
        assert!(matches!(ctx.state, GameState::Playing));
        assert_eq!(ctx.score, 0);
    }

    #[test]
    fn cannot_reverse_up_into_down() {
        let mut ctx = new_context();
        ctx.player_direction = PlayerDirection::Down;
        ctx.last_tick_direction = PlayerDirection::Down;
        ctx.move_up();
        assert_eq!(ctx.player_direction, PlayerDirection::Down);
    }

    #[test]
    fn cannot_reverse_down_into_up() {
        let mut ctx = new_context();
        ctx.player_direction = PlayerDirection::Up;
        ctx.last_tick_direction = PlayerDirection::Up;
        ctx.move_down();
        assert_eq!(ctx.player_direction, PlayerDirection::Up);
    }

    #[test]
    fn cannot_reverse_left_into_right() {
        let mut ctx = new_context();
        ctx.player_direction = PlayerDirection::Right;
        ctx.last_tick_direction = PlayerDirection::Right;
        ctx.move_left();
        assert_eq!(ctx.player_direction, PlayerDirection::Right);
    }

    #[test]
    fn cannot_reverse_right_into_left() {
        let mut ctx = new_context();
        ctx.player_direction = PlayerDirection::Left;
        ctx.last_tick_direction = PlayerDirection::Left;
        ctx.move_right();
        assert_eq!(ctx.player_direction, PlayerDirection::Left);
    }

    #[test]
    fn wall_collision_left() {
        let ctx = Context {
            player_position: vec![Point(1, 15)],
            board_size: Point(40, 30),
            ..new_context()
        };
        // Moving left into x=0 (wall)
        assert!(ctx.is_game_over(Point(0, 15)));
    }

    #[test]
    fn wall_collision_top() {
        let ctx = Context {
            player_position: vec![Point(10, 1)],
            board_size: Point(40, 30),
            ..new_context()
        };
        assert!(ctx.is_game_over(Point(10, 0)));
    }

    #[test]
    fn wall_collision_right() {
        let ctx = Context {
            player_position: vec![Point(38, 15)],
            board_size: Point(40, 30),
            ..new_context()
        };
        // x_size - 1 = 39, so x=39 is a wall
        assert!(ctx.is_game_over(Point(39, 15)));
    }

    #[test]
    fn wall_collision_bottom() {
        let ctx = Context {
            player_position: vec![Point(10, 28)],
            board_size: Point(40, 30),
            ..new_context()
        };
        // y_size - 1 = 29, so y=29 is a wall
        assert!(ctx.is_game_over(Point(10, 29)));
    }

    #[test]
    fn self_collision() {
        let ctx = Context {
            player_position: vec![Point(10, 10), Point(11, 10), Point(12, 10)],
            board_size: Point(40, 30),
            ..new_context()
        };
        // Head moves into the second segment
        assert!(ctx.is_game_over(Point(11, 10)));
    }

    #[test]
    fn no_collision_in_open_space() {
        let ctx = Context {
            player_position: vec![Point(10, 10)],
            board_size: Point(40, 30),
            ..new_context()
        };
        assert!(!ctx.is_game_over(Point(11, 10)));
    }

    #[test]
    fn food_spawns_not_on_snake() {
        let mut ctx = new_context();
        ctx.player_position = (0..10).map(|i| Point(5 + i, 15)).collect();
        ctx.spawn_food();
        assert!(ctx.food.is_some());
        let food = ctx.food.unwrap();
        assert!(!ctx.player_position.contains(&food));
    }

    #[test]
    fn food_not_spawned_on_walls() {
        let mut ctx = new_context();
        for _ in 0..100 {
            ctx.food = None;
            ctx.spawn_food();
            let food = ctx.food.unwrap();
            assert!(food.0 > 0 && food.0 < ctx.board_size.0 - 1);
            assert!(food.1 > 0 && food.1 < ctx.board_size.1 - 1);
        }
    }

    #[test]
    fn game_over_when_board_full() {
        let mut ctx = Context {
            board_size: Point(6, 5), // 4x3 = 12 interior cells
            player_position: vec![],
            ..new_context()
        };
        for x in 1..5 {
            for y in 1..4 {
                ctx.player_position.push(Point(x, y));
            }
        }
        ctx.state = GameState::Playing;
        ctx.food = None;
        ctx.spawn_food();
        assert!(matches!(ctx.state, GameState::Won));
        assert!(ctx.food.is_none());
    }

    #[test]
    fn eating_food_increases_score() {
        let mut ctx = new_context();
        ctx.toggle_pause();
        ctx.state = GameState::Playing;
        ctx.player_position = vec![Point(10, 10), Point(9, 10), Point(8, 10)];
        ctx.player_direction = PlayerDirection::Right;
        ctx.last_tick_direction = PlayerDirection::Right;
        ctx.food = Some(Point(11, 10));
        ctx.next_tick();
        assert_eq!(ctx.score, 1);
    }

    #[test]
    fn eating_food_grows_snake() {
        let mut ctx = new_context();
        ctx.toggle_pause();
        ctx.state = GameState::Playing;
        ctx.player_position = vec![Point(10, 10), Point(9, 10), Point(8, 10)];
        ctx.player_direction = PlayerDirection::Right;
        ctx.last_tick_direction = PlayerDirection::Right;
        ctx.food = Some(Point(11, 10));
        let len_before = ctx.player_position.len();
        ctx.next_tick();
        assert_eq!(ctx.player_position.len(), len_before + 1);
    }

    #[test]
    fn tick_does_nothing_when_paused() {
        let mut ctx = new_context();
        ctx.player_position = vec![Point(10, 10), Point(9, 10), Point(8, 10)];
        let pos_before = ctx.player_position.clone();
        ctx.next_tick();
        assert_eq!(ctx.player_position, pos_before);
    }

    #[test]
    fn tick_does_nothing_when_game_over() {
        let mut ctx = new_context();
        ctx.state = GameState::Over;
        ctx.player_position = vec![Point(10, 10)];
        let pos_before = ctx.player_position.clone();
        ctx.next_tick();
        assert_eq!(ctx.player_position, pos_before);
    }

    // --- speed curve tests ---

    #[test]
    fn tick_duration_starts_at_200ms() {
        let ctx = new_context();
        assert_eq!(ctx.tick_duration_ms(), 200);
    }

    #[test]
    fn tick_duration_decreases_with_score() {
        let mut ctx = new_context();
        ctx.score = 5;
        assert_eq!(ctx.tick_duration_ms(), 160);
    }

    #[test]
    fn tick_duration_respects_floor() {
        let mut ctx = new_context();
        ctx.score = 100;
        assert_eq!(ctx.tick_duration_ms(), 60);
    }

    #[test]
    fn tick_duration_at_exact_floor_boundary() {
        let mut ctx = new_context();
        ctx.score = 17;
        assert_eq!(ctx.tick_duration_ms(), 64);
        ctx.score = 18;
        assert_eq!(ctx.tick_duration_ms(), 60);
    }
}
