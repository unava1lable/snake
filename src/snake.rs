use crate::Direction;
use crate::GridPosition;
use ggez::{ graphics, Context, GameResult };
use std::collections::VecDeque;

// Snake的head与body的组成
struct Segment {
	pos: GridPosition
}

impl Segment {
	fn new(pos: GridPosition) -> Self {
		Self { pos }
	}
}

pub struct Snake {
	head: Segment,
	body: VecDeque<Segment>,
	dir: Direction,
	next_dir: Direction,
	last_update_dir: Option<Direction>,
}

impl Snake {
	pub fn new(pos: GridPosition) -> Self {
		let mut body = VecDeque::new();
		body.push_back(Segment::new((pos.x() - 1, pos.y()).into()));
		Self {
			head: Segment::new(pos),
			body,
			dir: Direction::Right,
			next_dir: Direction::Right,
			last_update_dir: None,
		}
	}

	pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
		for body_seg in self.body.iter() {
			let rectangle = graphics::Mesh::new_rectangle(
				ctx,
				graphics::DrawMode::fill(),
				body_seg.pos.into(),
				[0.3, 0.3, 0.0, 1.0].into())?;
			graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
		}

		let rectangle = graphics::Mesh::new_rectangle(
			ctx,
			graphics::DrawMode::fill(),
			self.head.pos.into(),
			[1.0, 0.5, 0.0, 1.0].into())?;
		graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
		Ok(())
	}
}