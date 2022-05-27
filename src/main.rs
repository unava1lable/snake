use ggez::{Context, ContextBuilder, GameResult};
use ggez::{conf, event, graphics};

const GRID_SIZE: (u16, u16) = (30, 20);
const GRID_CELL_SIZE: (u16, u16) = (32, 32);
const SCREEN_SIZE: (f32, f32) = ((GRID_SIZE.0 * GRID_CELL_SIZE.0) as f32,
                                 (GRID_SIZE.1 * GRID_CELL_SIZE.1) as f32);

mod grid;
mod snake;

pub use grid::{ GridPosition, Direction };
pub use snake::Snake;

struct GameState {
    snake: Snake,
}

impl GameState {
    fn new() -> Self {
        Self {
            snake: Snake::new((GRID_SIZE.0 / 4, GRID_SIZE.1 / 4).into())
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.snake.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [1.0, 1.0, 1.0, 1.0].into());
        self.snake.draw(ctx)?;
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
