use std::collections::HashMap;

use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::texture::{CompressedImageFormats, ImageType, TextureFormatPixelInfo};
use bevy_common_assets::json::JsonAssetPlugin;
use wgpu_types::Extent3d;

use crate::game_assets::keys::*;
use crate::helper::get_sub_rect;
use crate::spritesheet::aseprite_data::AsepriteData;
use crate::spritesheet::rectangles::Rect;

pub mod rectangles;
pub mod aseprite_data;

pub struct SpriteSheetPlugin;

impl Plugin for SpriteSheetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_asset::<SpriteSheet>()
            .add_plugin(JsonAssetPlugin::<AsepriteData>::new(&["aseprite.json"]))
            .add_startup_system(register_sheet_loader)
        ;
    }
}

/// Create the sheet loader and set every sheet grid.
fn register_sheet_loader(
    asset_loader: Res<AssetServer>
) {
    asset_loader.add_loader(
        SpriteSheetLoader::from_path_grid_iter(
            [
                (PACMAN_WALKING_UP, Grid::new(16, 16, 4, 1)),
                (PACMAN_WALKING_DOWN, Grid::new(16, 16, 4, 1)),
                (PACMAN_WALKING_LEFT, Grid::new(16, 16, 4, 1)),
                (PACMAN_WALKING_RIGHT, Grid::new(16, 16, 4, 1)),
                // (PACMAN_DYING, Grid::new(16, 16, 12, 1)),
                (BLINKY_UP, Grid::new(16, 16, 2, 1)),
                (BLINKY_DOWN, Grid::new(16, 16, 2, 1)),
                (BLINKY_LEFT, Grid::new(16, 16, 2, 1)),
                (BLINKY_RIGHT, Grid::new(16, 16, 2, 1)),
                (PINKY_UP, Grid::new(16, 16, 2, 1)),
                (PINKY_DOWN, Grid::new(16, 16, 2, 1)),
                (PINKY_LEFT, Grid::new(16, 16, 2, 1)),
                (PINKY_RIGHT, Grid::new(16, 16, 2, 1)),
                (INKY_UP, Grid::new(16, 16, 2, 1)),
                (INKY_DOWN, Grid::new(16, 16, 2, 1)),
                (INKY_LEFT, Grid::new(16, 16, 2, 1)),
                (INKY_RIGHT, Grid::new(16, 16, 2, 1)),
                (CLYDE_UP, Grid::new(16, 16, 2, 1)),
                (CLYDE_DOWN, Grid::new(16, 16, 2, 1)),
                (CLYDE_LEFT, Grid::new(16, 16, 2, 1)),
                (CLYDE_RIGHT, Grid::new(16, 16, 2, 1)),
                (FRIGHTENED, Grid::new(16, 16, 2, 1)),
                (FRIGHTENED_BLINKING, Grid::new(16, 16, 4, 1)),
                (OUTER_WALL_CORNER_BLINKING, Grid::new(16, 16, 2, 1)),
                (OUTER_WALL_BLINKING, Grid::new(16, 16, 2, 1)),
                (INNER_WALL_CORNER_BLINKING, Grid::new(16, 16, 2, 1)),
                (INNER_WALL_BLINKING, Grid::new(16, 16, 2, 1)),
                (GHOST_WALL_CORNER_BLINKING, Grid::new(16, 16, 2, 1)),
                (GHOST_WALL_BLINKING, Grid::new(16, 16, 2, 1)),
            ]
        )
    )
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

/// Loader for sprite sheets.
///
/// TODO: To load a sprite sheet, extra information is required ("where is which sprite?"). However, currently
///  the AssetLoader has no way to provide extra information to the loading process (for the same reason I invented this stupid .sheet extension).
///  Loading the grid data from files has some future, as software like aseprite generates this when creating sheets, but it will not work in WASM
///  and is not very flexible (what if sprite x uses another grid format?).
///  Therefore, this mapping from file paths to grids is the solution for now
pub struct SpriteSheetLoader {
    sheet_grids: HashMap<String, Grid>,
}

impl SpriteSheetLoader {
    fn from_path_grid_iter(iter: impl IntoIterator<Item=(impl ToString, Grid)>) -> Self {
        SpriteSheetLoader {
            sheet_grids: iter
                .into_iter()
                .map(|(ts, g)| (ts.to_string(), g))
                .collect()
        }
    }
}

const EXTENSIONS: &[&str] = &[
    "sheet.png"
];

impl AssetLoader for SpriteSheetLoader {
    /// Creates a sprite sheet from bytes of data, which was originally an image.
    ///
    /// The image itself is not enough to load the sheet. The information about where which sprite is is also required.
    /// This is provided by the loader constructor.
    ///
    /// TODO: Currently, only PNGs are supported.
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
        Box::pin(async move {
            let sheet_path = load_context.path().to_str().unwrap().to_string();
            let grid = self.sheet_grids.get(&sheet_path).expect("there should be a grid registered for this sheet");

            let image = Image::from_buffer(
                bytes,
                ImageType::Extension("png"),
                CompressedImageFormats::all(),
                true,
            )?;
            let textures = create_images(grid, &image)
                .into_iter()
                .enumerate()
                .map(|(i, img)| load_context.set_labeled_asset(&format!("{}_{}", sheet_path, i), LoadedAsset::new(img)))
                .collect::<Vec<_>>();
            load_context.set_default_asset(LoadedAsset::new(SpriteSheet::new(textures)));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        EXTENSIONS
    }
}

fn create_images(grid: &Grid, sheet_image: &Image) -> Vec<Image> {
    let mut images = vec![];
    for y in 0..grid.rows {
        for x in 0..grid.columns {
            let new_image = create_image(grid, sheet_image, x, y);
            images.push(new_image)
        }
    }
    images
}

fn create_image(grid: &Grid, sheet_image: &Image, column: usize, row: usize) -> Image {
    let size = Extent3d {
        width: grid.width as u32,
        height: grid.height as u32,
        depth_or_array_layers: sheet_image.texture_descriptor.size.depth_or_array_layers,
    };
    let dimension = sheet_image.texture_descriptor.dimension;
    let format = sheet_image.texture_descriptor.format;

    let sheet_width = sheet_image.texture_descriptor.size.width as usize * format.pixel_size();
    let width = grid.width * format.pixel_size();
    let height = grid.height;
    let data = get_sub_rect(sheet_image.data.as_slice(), sheet_width, column, row, width, height);

    Image::new(
        size,
        dimension,
        data,
        format,
    )
}

#[derive(Copy, Clone)]
struct Grid {
    width: usize,
    height: usize,
    columns: usize,
    rows: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize, columns: usize, rows: usize) -> Self {
        Self { width, height, columns, rows }
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
    use crate::spritesheet::{extract_rectangle, split_image_by_rectangles};
    use crate::spritesheet::rectangles::Rect;

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