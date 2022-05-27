use crate::GridPosition;
use ggez::{ Context, GameResult, graphics };

pub struct Food {
	pub pos: GridPosition,
}

impl Food {
	pub fn new(pos: GridPosition) -> Self {
		Self {
			pos
		}
	}

	pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
		let rectangle = graphics::Mesh::new_rectangle(
			ctx,
			graphics::DrawMode::fill(),
			self.pos.into(),
			[0.0, 0.0, 1.0, 1.0].into()
		)?;
		graphics::draw(ctx, &rectangle, (ggez::mint::Point2{ x: 0.0, y: 0.0 },))?;

		Ok(())
	}
}

#[derive(Clone, Copy, Debug)]
pub enum Ate {
	Food,
	Snake,
}