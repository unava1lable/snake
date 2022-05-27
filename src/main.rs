use getrandom;
use ggez::{Context, ContextBuilder, GameResult, conf, event, graphics};
use oorandom::Rand32;

const GRID_SIZE: (i16, i16) = (30, 20);
const GRID_CELL_SIZE: (i16, i16) = (32, 32);
const SCREEN_SIZE: (f32, f32) = ((GRID_SIZE.0 * GRID_CELL_SIZE.0) as f32,
                                 (GRID_SIZE.1 * GRID_CELL_SIZE.1) as f32);

mod food;                                 
mod grid;
mod snake;

pub use food::Food;
pub use grid::{ GridPosition, Direction };
pub use snake::Snake;

struct GameState {
    snake: Snake,
    food: Food,
    rng: Rand32
}

impl GameState {
    fn new() -> Self {
        let snake_pos: GridPosition = (GRID_SIZE.0 / 2, GRID_SIZE.1 / 2).into();
        let mut seed: [u8; 8] = [0; 8];
        getrandom::getrandom(&mut seed).expect("Failed to create rng seed");
        let mut rng = Rand32::new(u64::from_ne_bytes(seed));
        let food_pos = GridPosition::random(&mut rng, GRID_SIZE.0, GRID_SIZE.1);

        Self {
            snake: Snake::new(snake_pos),
            food: Food::new(food_pos),
            rng,
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while ggez::timer::check_update_time(ctx, 4) {
            self.snake.update(); 
        }
        
        Ok(()) 
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [1.0, 1.0, 1.0, 1.0].into());
        self.snake.draw(ctx)?;
        self.food.draw(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: event::KeyCode, _keymods: event::KeyMods, _repeat: bool) {
        if let Some(dir) = Direction::from_key(keycode) {
            if self.snake.dir != self.snake.last_update_dir && self.snake.dir != dir.inverse() {
                self.snake.next_dir = Some(dir);
            } else if self.snake.last_update_dir != dir.inverse() {
                self.snake.dir = dir;
            }
        }
    }
}

fn main() -> GameResult<()> {
    let (ctx, event_loop) = ContextBuilder::new("Snake", "Jerry")
        .window_setup(conf::WindowSetup::default().title("Snake"))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;
    
    let game_state = GameState::new();

    event::run(ctx, event_loop, game_state)
}
