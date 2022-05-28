use crate::Ate;
use crate::Direction;
use crate::Food;
use crate::GridPosition;
use ggez::{ graphics, Context, GameResult };
use std::collections::VecDeque;

// 组成Snake的head与body的组件
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
	ate: Option<Ate>,
	dir: Direction,
	next_dir: Option<Direction>,
	last_update_dir: Direction,
}

impl Snake {
	pub fn new(pos: GridPosition) -> Self {
		let mut body = VecDeque::new();
		body.push_back(Segment::new((pos.x() - 1, pos.y()).into()));
		Self {
			head: Segment::new(pos),
			body,
			ate: None,
			dir: Direction::Right,
			next_dir: None,
			last_update_dir: Direction::Right,
		}
	}

	fn eat_snake(&self) -> bool {
		for body_seg in self.body.iter() {
			if self.head.pos == body_seg.pos {
				return true;
			}
		}
		false
	}

	fn eat_food(&self, food: &Food) -> bool {
		self.head.pos == food.pos()
	}

	pub fn update(&mut self, food: &Food) {
		if self.last_update_dir == self.dir && self.next_dir.is_some() {
			self.dir = self.next_dir.unwrap();
			self.next_dir = None;
		}

		let new_head_pos = GridPosition::new_from_move(self.head.pos, self.dir);
		let new_head = Segment::new(new_head_pos);
		self.body.push_front(self.head);
		self.head = new_head;
		
		if self.eat_snake() {
			self.ate = Some(Ate::Snake)
		} else if self.eat_food(food) {
			self.ate = Some(Ate::Food)
		} else {
			self.ate = None
		}

		if self.ate.is_none() {
			self.body.pop_back();
		}

		self.last_update_dir = self.dir;
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

	pub fn ate(&self) -> Option<Ate> {
		self.ate
	}

	pub fn dir(&self) -> Direction {
		self.dir
	}

	pub fn next_dir(&self) -> Option<Direction> {
		self.next_dir
	}

	pub fn last_update_dir(&self) -> Direction {
		self.last_update_dir
	}

	pub fn set_dir(&mut self, dir: Direction) {
		self.dir = dir;
	}

	pub fn set_next_dir(&mut self, dir: Direction) {
		self.next_dir = Some(dir);
	}
}