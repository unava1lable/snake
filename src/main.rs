use ggez::{ContextBuilder, GameResult, event, conf};

const SCREEN_SIZE: (f32, f32) = (960.0, 680.0);

struct GameState {

}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> GameResult {
        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("Snake", "Jerry")
        .window_setup(conf::WindowSetup::default().title("Snake"))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;
    
    let game_state = GameState {};

    event::run(ctx, event_loop, game_state)
}
