use ggez::{Context, ContextBuilder, GameResult};
use ggez::{conf, event, graphics};
use oorandom::Rand32;

const SCREEN_SIZE: (f32, f32) = (960.0, 680.0);

struct GridPosition {
    x: u16,
    y: u16,
}

impl From<(u16, u16)> for GridPosition {
    fn from(pos: (u16, u16)) -> Self {
        Self {
            x: pos.0,
            y: pos.1
        }
    }
}

impl GridPosition {
    fn new(x: u16, y: u16) -> Self {
        Self {
            x,
            y,
        }
    }

    fn random(rng: &mut Rand32, max_x: u16, max_y: u16) -> Self {
        (
            rng.rand_range(0..max_x as u32) as u16,
            rng.rand_range(0..max_y as u32) as u16
        ).into()
    }
}

struct GameState {}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [1.0, 1.0, 1.0, 1.0].into());
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult<()> {
    let (ctx, event_loop) = ContextBuilder::new("Snake", "Jerry")
        .window_setup(conf::WindowSetup::default().title("Snake"))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;
    
    let game_state = GameState {};

    event::run(ctx, event_loop, game_state)
}
