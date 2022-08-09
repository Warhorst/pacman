use serde::Deserialize;

use std::collections::BTreeMap;
use crate::common::position::Position;
use crate::spritesheet::rectangles::{Rect, RectIter};

/// Represents the json data for a sprite sheet that can be generated when exporting a sheet.
/// Used to load sheets from images using the data from the json file.
#[derive(Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "187ce97b-0f53-4bf6-824d-b5f8289c4bfe"]
pub struct AsepriteData {
    frames: BTreeMap<String, FrameValue>
}

impl IntoIterator for AsepriteData {
    type Item = Rect;
    type IntoIter = RectIter;

    fn into_iter(self) -> Self::IntoIter {
        RectIter::new(
            self.frames
                .values()
                .map(|fv| fv.frame)
                .map(|f| Rect::new(Position::new(f.x as isize, f.y as isize), f.w, f.h))
        )
    }
}

impl IntoIterator for &AsepriteData {
    type Item = Rect;
    type IntoIter = RectIter;

    fn into_iter(self) -> Self::IntoIter {
        RectIter::new(
            self.frames
                .values()
                .map(|fv| fv.frame)
                .map(|f| Rect::new(Position::new(f.x as isize, f.y as isize), f.w, f.h))
        )
    }
}

#[derive(Deserialize)]
struct FrameValue {
    frame: Frame
}

#[derive(Copy, Clone, Deserialize)]
struct Frame {
    x: usize,
    y: usize,
    w: usize,
    h: usize
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;
    use crate::common::position::Position;
    use crate::spritesheet::aseprite_data::AsepriteData;
    use crate::spritesheet::rectangles::Rect;

    #[test]
    fn can_be_deserialized() {
        let file = File::open("./test/blinky_left.json").unwrap();
        let reader = BufReader::new(file);
        let aseprite_data: AsepriteData = serde_json::from_reader(reader).unwrap();

        assert_eq!(aseprite_data.frames.len(), 2);

        let sizes = aseprite_data.frames.values().map(|f| f.frame).collect::<Vec<_>>();
        let size_a = sizes[0];
        let size_b = sizes[1];

        let (x, y, w, h) = (size_a.x, size_a.y, size_a.w, size_a.h);
        assert_eq!((x,y,w,h), (0, 0, 16, 16));
        let (x, y, w, h) = (size_b.x, size_b.y, size_b.w, size_b.h);
        assert_eq!((x,y,w,h), (16, 0, 16, 16))
    }

    #[test]
    fn can_be_converted_into_an_iterator_of_rectangles() {
        let file = File::open("./test/blinky_left.json").unwrap();
        let reader = BufReader::new(file);
        let aseprite_data: AsepriteData = serde_json::from_reader(reader).unwrap();

        let rectangles = aseprite_data.into_iter().collect::<Vec<_>>();

        assert_eq!(rectangles.len(), 2);
        assert_eq!(rectangles[0], Rect::new(Position::new(0, 0), 16, 16));
        assert_eq!(rectangles[1], Rect::new(Position::new(16, 0), 16, 16));
    }
}