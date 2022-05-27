use crate::Direction;
use crate::GridPosition;
use ggez::{ graphics, Context, GameResult };
use std::collections::VecDeque;

// 组成Snake的head与body的组件
#[derive(Clone, Copy, Debug)]
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
	pub dir: Direction,
	pub next_dir: Option<Direction>,
	pub last_update_dir: Direction,
}

impl Snake {
	pub fn new(pos: GridPosition) -> Self {
		let mut body = VecDeque::new();
		body.push_back(Segment::new((pos.x() - 1, pos.y()).into()));
		Self {
			head: Segment::new(pos),
			body,
			dir: Direction::Right,
			next_dir: None,
			last_update_dir: Direction::Right,
		}
	}

	pub fn update(&mut self) {
		if self.last_update_dir == self.dir && self.next_dir.is_some() {
			self.dir = self.next_dir.unwrap();
			self.next_dir = None;
		}

		let new_head_pos = GridPosition::new_from_move(self.head.pos, self.dir);
		let new_head = Segment::new(new_head_pos);
		self.body.push_front(self.head);
		self.head = new_head;
		self.body.pop_back();
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