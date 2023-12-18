#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct Beam {
    pub direction: Direction,
    pub x: usize,
    pub y: usize,
}
