use ggez::{ContextBuilder, GameResult, Context, event, conf, graphics};

const SCREEN_SIZE: (f32, f32) = (960.0, 680.0);

struct GameState {

}

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
