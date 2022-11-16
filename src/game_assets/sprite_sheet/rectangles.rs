use crate::game::position::Position;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rect {
    pub position: Position,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    pub fn new(position: Position, width: usize, height: usize) -> Self {
        Self { position, width, height }
    }
}

pub struct RectIter {
    index: usize,
    rects: Vec<Rect>
}

impl RectIter {
    pub fn new(rects: impl IntoIterator<Item=Rect>) -> Self {
        RectIter {
            index: 0,
            rects: rects.into_iter().collect()
        }
    }
}

impl Iterator for RectIter {
    type Item = Rect;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index < self.rects.len() {
            true => {
                let value = self.rects[self.index];
                self.index += 1;
                Some(value)
            },
            false => None
        }
    }
}