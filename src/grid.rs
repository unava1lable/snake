#![allow(unused)]
use oorandom::Rand32;
use ggez::graphics;
use crate::{ GRID_SIZE, GRID_CELL_SIZE };

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridPosition {
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

impl From<GridPosition> for graphics::Rect {
	fn from(pos: GridPosition) -> Self {
		graphics::Rect::new_i32(
			pos.x as i32 * GRID_CELL_SIZE.0 as i32,
			pos.y as i32 * GRID_CELL_SIZE.1 as i32,
			GRID_CELL_SIZE.0 as i32,
			GRID_CELL_SIZE.1 as i32)
	}
}

impl GridPosition {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn random(rng: &mut Rand32, max_x: u16, max_y: u16) -> Self {
        (
            rng.rand_range(0..max_x as u32) as u16,
            rng.rand_range(0..max_y as u32) as u16
        ).into()
    }

	pub fn new_from_move(pos: GridPosition, dir: Direction) -> Self {
		match dir {
			Direction::Up => GridPosition::new(pos.x, (pos.y - 1) % GRID_SIZE.1),
			Direction::Down => GridPosition::new(pos.x, (pos.y + 1) % GRID_SIZE.1),
			Direction::Left => GridPosition::new((pos.x - 1) % GRID_SIZE.0, pos.y),
			Direction::Right => GridPosition::new((pos.x + 1) % GRID_SIZE.0, pos.y),
		}
	}

	pub fn x(&self) -> u16 {
		self.x
	}

	pub fn y(&self) -> u16 {
		self.y
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}