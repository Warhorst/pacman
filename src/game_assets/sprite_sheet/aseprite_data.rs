use std::cmp::Ordering::*;
use std::collections::HashMap;
use bevy::reflect::TypePath;
use serde::Deserialize;

use crate::game::position::Position;
use crate::game_assets::sprite_sheet::rectangles::{Rect, RectIter};

/// Represents the json data for a sprite sheet that can be generated when exporting a sheet.
/// Used to load sheets from images using the data from the json file.
#[derive(Deserialize, bevy::reflect::TypeUuid, TypePath)]
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