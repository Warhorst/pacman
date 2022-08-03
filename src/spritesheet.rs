use std::collections::HashMap;
use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::texture::{CompressedImageFormats, ImageType, TextureFormatPixelInfo};
use wgpu_types::Extent3d;
use crate::helper::get_sub_rect;

pub struct SpriteSheetPlugin;

impl Plugin for SpriteSheetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_asset::<SpriteSheet>()
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
                ("textures/pacman/pacman_walking_up.png.sheet", Grid::new(16,16,4,1)),
                ("textures/pacman/pacman_walking_down.png.sheet", Grid::new(16,16,4,1)),
                ("textures/pacman/pacman_walking_left.png.sheet", Grid::new(16,16,4,1)),
                ("textures/pacman/pacman_walking_right.png.sheet", Grid::new(16,16,4,1)),
                ("textures/pacman/pacman_dying.png.sheet", Grid::new(16,16,12,1)),
                ("textures/walls/outer_wall_corner_blinking.png.sheet", Grid::new(16, 16, 2, 1)),
                ("textures/walls/outer_wall_blinking.png.sheet", Grid::new(16, 16, 2, 1)),
                ("textures/walls/inner_wall_corner_blinking.png.sheet", Grid::new(16, 16, 2, 1)),
                ("textures/walls/inner_wall_blinking.png.sheet", Grid::new(16, 16, 2, 1)),
                ("textures/walls/ghost_house_wall_corner_blinking.png.sheet", Grid::new(16, 16, 2, 1)),
                ("textures/walls/ghost_house_wall_blinking.png.sheet", Grid::new(16, 16, 2, 1)),
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
    pub fn new(textures: Vec<Handle<Image>>) -> Self {
        Self { textures }
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
    sheet_grids: HashMap<String, Grid>
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
    "sheet",
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
                .collect();
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