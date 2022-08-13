use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::texture::TextureFormatPixelInfo;
use bevy_common_assets::json::JsonAssetPlugin;
use wgpu_types::Extent3d;

use crate::sprite_sheet::aseprite_data::AsepriteData;
use crate::sprite_sheet::rectangles::Rect;

pub mod rectangles;
pub mod aseprite_data;

pub struct SpriteSheetPlugin;

impl Plugin for SpriteSheetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_asset::<SpriteSheet>()
            .add_plugin(JsonAssetPlugin::<AsepriteData>::new(&["aseprite.json"]))
        ;
    }
}

/// A loaded sprite sheet with handles to all loaded sub images.
#[derive(TypeUuid)]
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

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use wgpu_types::{Extent3d, TextureDimension, TextureFormat};

    use crate::common::position::Position;
    use crate::sprite_sheet::{extract_rectangle, split_image_by_rectangles};
    use crate::sprite_sheet::rectangles::Rect;

    #[test]
    fn a_vector_of_bytes_can_be_split_by_rectangles() {
        let data = vec![
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1, 1, 0, 0, 0, 0,
            0, 0, 1, 1, 0, 0, 0, 0,
            0, 0, 0, 0, 2, 2, 2, 2,
            0, 0, 0, 0, 2, 2, 2, 2,
        ];

        let pixel_width = 1;
        assert_eq!(extract_rectangle(data.as_slice(), Rect::new(Position::new(0, 0), 8, 6), 8, pixel_width), data);
        assert_eq!(extract_rectangle(data.as_slice(), Rect::new(Position::new(2, 2), 2, 2), 8, pixel_width), vec![1; 4]);
        assert_eq!(extract_rectangle(data.as_slice(), Rect::new(Position::new(4, 4), 4, 2), 8, pixel_width), vec![2; 8]);
    }

    #[test]
    fn an_image_can_be_split_by_rectangles() {
        // a 4 by 2 pixel image in RGBA -> 4 * 4 * 2 = 32 len
        let data = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1,
            0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1,
        ];

        let image = Image::new(
            Extent3d {
                width: 4,
                height: 2,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            data,
            TextureFormat::Rgba8Uint,
        );

        let sub_images = split_image_by_rectangles(&image, [
            Rect::new(Position::new(0, 0), 2, 2),
            Rect::new(Position::new(2, 0), 2, 2)
        ]).into_iter().collect::<Vec<_>>();

        let expected_one = Image::new(
            Extent3d {
                width: 2,
                height: 2,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            vec![0; 16],
            TextureFormat::Rgba8Uint,
        );

        let expected_two = Image::new(
            Extent3d {
                width: 2,
                height: 2,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            vec![1; 16],
            TextureFormat::Rgba8Uint,
        );

        assert_eq!(sub_images.len(), 2);
        assert_eq!(sub_images[0].data, expected_one.data);
        assert_eq!(sub_images[0].texture_descriptor.size, expected_one.texture_descriptor.size);
        assert_eq!(sub_images[1].data, expected_two.data);
        assert_eq!(sub_images[1].texture_descriptor.size, expected_two.texture_descriptor.size);
    }
}