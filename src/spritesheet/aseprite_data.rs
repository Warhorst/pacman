use std::cmp::Ordering::*;
use std::collections::HashMap;
use serde::Deserialize;

use crate::common::position::Position;
use crate::spritesheet::rectangles::{Rect, RectIter};

/// Represents the json data for a sprite sheet that can be generated when exporting a sheet.
/// Used to load sheets from images using the data from the json file.
#[derive(Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "187ce97b-0f53-4bf6-824d-b5f8289c4bfe"]
pub struct AsepriteData {
    frames: HashMap<String, FrameValue>,
}

impl AsepriteData {
    /// Returns an rectangle iterator for this aseprite data.
    /// All sub sprite names have a name like "<sheet name> <n>.aseprite", where
    /// <sheet name> is the name of the sprite sheet and <n> is the index.
    /// To return an ordered iterator, the entries must be sorted by key regarding length
    /// and name.
    pub fn rect_iter(&self) -> RectIter {
        let mut frames_vec = self.frames.iter().collect::<Vec<_>>();
        frames_vec.sort_by(|(ka, _), (kb, _)| match ka.len().cmp(&kb.len()) {
            Less => Less,
            Greater => Greater,
            Equal => ka.cmp(&kb)
        });

        RectIter::new(
            frames_vec
                .into_iter()
                .map(|(_, fv)| fv.frame)
                .map(|f| Rect::new(Position::new(f.x as isize, f.y as isize), f.w, f.h))
        )
    }
}

#[derive(Deserialize)]
struct FrameValue {
    frame: Frame,
}

#[derive(Copy, Clone, Deserialize, Eq, PartialEq)]
struct Frame {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;
    use crate::common::position::Position;
    use crate::spritesheet::aseprite_data::{AsepriteData, Frame};
    use crate::spritesheet::rectangles::Rect;

    #[test]
    fn it_can_be_deserialized() {
        let file = File::open("./test/blinky_left.json").unwrap();
        let reader = BufReader::new(file);
        let aseprite_data: AsepriteData = serde_json::from_reader(reader).unwrap();

        assert_eq!(aseprite_data.frames.len(), 2);

        let sizes = aseprite_data.frames.values().map(|f| f.frame).collect::<Vec<_>>();

        assert!(sizes.contains(&Frame { x: 0, y: 0, w: 16, h: 16 }));
        assert!(sizes.contains(&Frame { x: 16, y: 0, w: 16, h: 16 }));
    }

    #[test]
    fn it_can_provide_an_ordered_iterator_of_rectangles() {
        let file = File::open("./test/pacman_dying.aseprite.json").unwrap();
        let reader = BufReader::new(file);
        let aseprite_data: AsepriteData = serde_json::from_reader(reader).unwrap();

        let rectangles = aseprite_data.rect_iter().collect::<Vec<_>>();

        assert_eq!(rectangles.len(), 12);

        for i in 0..12 {
            assert_eq!(rectangles[i], Rect::new(Position::new(i as isize * 16, 0), 16, 16));
        }
    }
}