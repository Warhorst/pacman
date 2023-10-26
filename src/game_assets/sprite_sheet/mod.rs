use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use bevy::render::texture::TextureFormatPixelInfo;
use bevy_common_assets::json::JsonAssetPlugin;
use wgpu_types::Extent3d;

use crate::game_assets::sprite_sheet::aseprite_data::AsepriteData;
use crate::game_assets::sprite_sheet::rectangles::Rect;

pub mod rectangles;
pub mod aseprite_data;

pub (in crate::game_assets) struct SpriteSheetPlugin;

impl Plugin for SpriteSheetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_asset::<SpriteSheet>()
            .add_plugins(JsonAssetPlugin::<AsepriteData>::new(&["aseprite.json"]))
        ;
    }
}

/// A loaded sprite sheet with handles to all loaded sub images.
#[derive(TypeUuid, TypePath)]
#[uuid = "997f1174-eb67-4d02-8ee6-fb41c987bb18"]
pub struct SpriteSheet {
    pub textures: Vec<Handle<Image>>,
}

impl SpriteSheet {
    pub fn new(handles: impl IntoIterator<Item=Handle<Image>>) -> Self {
        Self {
            textures: handles.into_iter().collect()
        }
    }

    pub fn image_at(&self, index: usize) -> Handle<Image> {
        self.textures[index].clone()
    }

    pub fn images_at(&self, indexes: impl IntoIterator<Item=usize>) -> Vec<Handle<Image>> {
        indexes.into_iter().map(|i| self.textures[i].clone()).collect()
    }
}

/// Split a given image by the given iterator of rectangles and create sub images from it.
pub fn split_image_by_rectangles<'a>(image: &'a Image, rectangles: impl IntoIterator<Item=Rect> + 'a) -> impl IntoIterator<Item=Image> + 'a {
    let dimension = image.texture_descriptor.dimension;
    let format = image.texture_descriptor.format;
    let sheet_width = image.texture_descriptor.size.width as usize * format.pixel_size();

    rectangles
        .into_iter()
        .map(move |rect| {
            let size = Extent3d {
                width: rect.width as u32,
                height: rect.height as u32,
                depth_or_array_layers: image.texture_descriptor.size.depth_or_array_layers,
            };

            let data = extract_rectangle(image.data.as_slice(), rect, sheet_width, format.pixel_size());
            Image::new(
                size,
                dimension,
                data,
                format,
            )
        })
}

fn extract_rectangle(data: &[u8], rect: Rect, data_width: usize, pixel_width: usize) -> Vec<u8> {
    let mut extracted = Vec::with_capacity(rect.width * rect.height);
    let start_index = data_width * rect.position.y as usize;

    for y in 0..rect.height {
        let start = start_index + y * data_width + rect.position.x as usize * pixel_width;
        let end = start + rect.width * pixel_width;
        data[start..end].into_iter().for_each(|val| extracted.push(*val))
    }

    extracted
}