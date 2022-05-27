use oorandom::Rand32;

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
}