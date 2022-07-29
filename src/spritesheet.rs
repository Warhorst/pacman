use std::fs::File;
use std::path::PathBuf;
use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::texture::{CompressedImageFormats, ImageType, TextureFormatPixelInfo};
use wgpu_types::Extent3d;
use crate::helper::get_sub_rect;
use serde::Deserialize;

pub struct SpriteSheetPlugin;

impl Plugin for SpriteSheetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_asset::<SpriteSheet>()
            .add_startup_system(register_sheet_loader)
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
    pub fn new(textures: Vec<Handle<Image>>) -> Self {
        Self { textures }
    }
}

pub struct SpriteSheetLoader;

const EXTENSIONS: &[&str] = &[
    "sheet",
];

impl AssetLoader for SpriteSheetLoader {
    /// Creates a sprite sheet from bytes of data, which was originally an image.
    ///
    /// The image itself is not enough to load the sheet. The information about where which sprite is is also required.
    /// This is provided by an extra json file in the same directory.
    ///
    /// TODO: Currently only grids are provided. A vector of rectangles or similar would be better
    /// TODO: Currently, only PNGs are supported.
    /// TODO: There are asset labels, like in the gltf example (https://bevyengine.org/news/bevy-0-7/#gltf-animation-importing), but i dont know how to use them. Maybe these could be used to provide the grid via string rather than file
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
        Box::pin(async move {
            let mut data_file_path = PathBuf::from(load_context.path());
            data_file_path.set_extension("json");
            let grid_file_path = format!("./assets/{}", data_file_path.to_str().unwrap());
            let grid_file = File::open(grid_file_path).unwrap();
            let grid: Grid = serde_json::from_reader(grid_file).unwrap();

            let image = Image::from_buffer(
                bytes,
                ImageType::Extension("png"),
                CompressedImageFormats::all(),
                true,
            )?;
            let textures = create_images(grid, &image)
                .into_iter()
                .enumerate()
                .map(|(i, img)| load_context.set_labeled_asset(&format!("{}_{}", load_context.path().to_str().unwrap(), i), LoadedAsset::new(img)))
                .collect();
            load_context.set_default_asset(LoadedAsset::new(SpriteSheet::new(textures)));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        EXTENSIONS
    }
}

fn create_images(grid: Grid, sheet_image: &Image) -> Vec<Image> {
    let mut images = vec![];
    for y in 0..grid.rows {
        for x in 0..grid.columns {
            let new_image = create_image(grid, sheet_image, x, y);
            images.push(new_image)
        }
    }
    images
}

fn create_image(grid: Grid, sheet_image: &Image, column: usize, row: usize) -> Image {
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

#[derive(Copy, Clone, Deserialize)]
struct Grid {
    width: usize,
    height: usize,
    columns: usize,
    rows: usize,
}

fn register_sheet_loader(
    asset_loader: Res<AssetServer>
) {
    asset_loader.add_loader(SpriteSheetLoader)
}