#![allow(unused)]
use oorandom::Rand32;
use ggez::graphics;
use ggez::event::KeyCode;
use crate::{ GRID_SIZE, GRID_CELL_SIZE };

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridPosition {
    x: i16,
    y: i16,
}

impl From<(i16, i16)> for GridPosition {
    fn from(pos: (i16, i16)) -> Self {
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
    pub fn new(x: i16, y: i16) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn random(rng: &mut Rand32, max_x: i16, max_y: i16) -> Self {
        (
            rng.rand_range(0..max_x as u32) as i16,
            rng.rand_range(0..max_y as u32) as i16
        ).into()
    }

	pub fn new_from_move(pos: GridPosition, dir: Direction) -> Self {
		match dir {
			Direction::Up => GridPosition::new(pos.x, (pos.y - 1).rem_euclid(GRID_SIZE.1)),
			Direction::Down => GridPosition::new(pos.x, (pos.y + 1).rem_euclid(GRID_SIZE.1)),
			Direction::Left => GridPosition::new((pos.x - 1).rem_euclid(GRID_SIZE.0), pos.y),
			Direction::Right => GridPosition::new((pos.x + 1).rem_euclid(GRID_SIZE.0), pos.y),
		}
	}

	pub fn x(&self) -> i16 {
		self.x
	}

	pub fn y(&self) -> i16 {
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

impl Direction {
	pub fn inverse(&self) -> Self {
		match *self {
			Self::Up => Self::Down,
			Self::Down => Self::Up,
			Self::Left => Self::Right,
			Self::Right => Self::Left,
		}
	}

	pub fn from_key(key: KeyCode) -> Option<Self> {
		match key {
			KeyCode::W | KeyCode::Up => Some(Self::Up),
			KeyCode::S | KeyCode::Down => Some(Self::Down),
			KeyCode::A | KeyCode::Left => Some(Self::Left),
			KeyCode::D | KeyCode::Right => Some(Self::Right),
			_ => None,
		}
	}
}