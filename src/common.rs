#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq)]
pub struct Position(usize, usize);

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position(x, y)
    }

    pub fn x(&self) -> usize {
        self.0
    }

    pub fn y(&self) -> usize {
        self.1
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}