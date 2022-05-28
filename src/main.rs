use getrandom;
use ggez::{Context, ContextBuilder, GameResult, conf, event, graphics};
use oorandom::Rand32;
use std::{ time, thread };

const GRID_SIZE: (i16, i16) = (30, 20);
const GRID_CELL_SIZE: (i16, i16) = (32, 32);
const SCREEN_SIZE: (f32, f32) = ((GRID_SIZE.0 * GRID_CELL_SIZE.0) as f32,
                                 (GRID_SIZE.1 * GRID_CELL_SIZE.1) as f32);

mod food;                                 
mod grid;
mod snake;

pub use food::{ Food, Ate };
pub use grid::{ GridPosition, Direction };
pub use snake::Snake;

struct GameState {
    snake: Snake,
    food: Food,
    rng: Rand32,
    running: bool,
    show_starting_menu: Option<String>,
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
            running: true,
            show_starting_menu: Some("Starting: ".to_string()),
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while ggez::timer::check_update_time(ctx, 4) {
            if self.running {
                self.snake.update(&self.food);
                if let Some(food) = self.snake.ate() {
                    match food {
                        Ate::Food => {
                            let food_pos = GridPosition::random(&mut self.rng, GRID_SIZE.0, GRID_SIZE.1);
                            self.food = Food::new(food_pos);
                        },
                        Ate::Snake => self.running = false,
                    }
                }
            }
        }
        
        Ok(()) 
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [1.0, 1.0, 1.0, 1.0].into());
        if let Some(ref text) = &self.show_starting_menu {
            for secs in (1..=3).rev() {
                graphics::clear(ctx, [1.0, 1.0, 1.0, 1.0].into());
                let text = format!("{}{}", text, secs);
                let text_fragment = graphics::TextFragment::new(text).color([0.0, 0.0, 0.0, 1.0]);
                graphics::draw(ctx, &(graphics::Text::new(text_fragment)), (ggez::mint::Point2{x: 0.0, y: 0.0},))?;
                graphics::present(ctx)?;
                thread::sleep(time::Duration::from_secs(1));
            }
            self.show_starting_menu = None;
        }
        self.snake.draw(ctx)?;            
        self.food.draw(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: event::KeyCode, _keymods: event::KeyMods, _repeat: bool) {
        if let Some(dir) = Direction::from_key(keycode) {
            if self.snake.dir() != self.snake.last_update_dir() && self.snake.dir() != dir.inverse() {
                self.snake.set_next_dir(dir);
            } else if self.snake.last_update_dir() != dir.inverse() {
                self.snake.set_dir(dir);
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
