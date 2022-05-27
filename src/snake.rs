use crate::GridPosition;

struct Segment {
	pos: GridPosition
}

impl Segment {
	fn new(pos: GridPosition) -> Self {
		Self { pos }
	}
}

struct Snake {}