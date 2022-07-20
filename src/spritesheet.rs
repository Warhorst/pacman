use std::collections::HashMap;
use std::ops::DerefMut;
use bevy::asset::HandleId;
use bevy::prelude::*;
use bevy::render::texture::TextureFormatPixelInfo;
use wgpu_types::Extent3d;
use crate::helper::get_sub_rect;

pub struct SpriteSheetPlugin;

impl Plugin for SpriteSheetPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SpriteSheets::new())
            .add_system(update_sprite_sheets)
        ;
    }
}

/// Reads image asset events and updates sprite sheets associated with it.
fn update_sprite_sheets(
    mut image_assets: ResMut<Assets<Image>>,
    mut sprite_sheets: ResMut<SpriteSheets>,
    mut asset_events: EventReader<AssetEvent<Image>>,
) {
    for event in asset_events.iter() {
        let handle = match event {
            AssetEvent::Created { handle } => handle,
            AssetEvent::Modified { handle } => handle,
            _ => return
        };

        if let Some(sheet) = sprite_sheets.get_sheet_mut_for_handle(handle.id) {
            let image = image_assets.get(handle.id).unwrap().clone();
            sheet.create_textures(&image, image_assets.deref_mut())
        }
    }
}

pub struct SpriteSheets {
    sheets: HashMap<HandleId, SpriteSheet>,
}

impl SpriteSheets {
    pub fn new() -> Self {
        SpriteSheets {
            sheets: HashMap::new()
        }
    }

    /// Creates an new sprite sheet from the given data.
    ///
    /// How this works:
    /// The given dimension, columns and index describe the structure of the sprite sheet. The handle is a pointer to the sprite sheet image.
    /// The sprite sheet cannot be split in seperate images, as it is not loaded yet. The sprite sheet is therefore filled with columns * rows default
    /// images.
    /// When the sheet is loaded (which is checked in update_sprite_sheets), the sheet is split and all handles get updated.
    /// TODO: This is more or less a hack. Rot reloading does not work and probably other stuff.
    pub fn add_sheet(&mut self, sheet_handle: Handle<Image>, image_assets: &mut Assets<Image>, sprite_dimension: Vec2, columns: usize, rows: usize) -> &SpriteSheet {
        let sheet = SpriteSheet::new(image_assets, sprite_dimension, columns, rows);
        self.sheets.insert(sheet_handle.id, sheet);
        self.sheets.get(&sheet_handle.id).unwrap()
    }

    fn get_sheet_mut_for_handle(&mut self, handle_id: HandleId) -> Option<&mut SpriteSheet> {
        self.sheets.get_mut(&handle_id)
    }
}

pub struct SpriteSheet {
    textures: Vec<Handle<Image>>,
    sprite_dimension: Vec2,
    columns: usize,
    rows: usize,
}

impl SpriteSheet {
    fn new(image_assets: &mut Assets<Image>, sprite_dimension: Vec2, columns: usize, rows: usize) -> Self {
        SpriteSheet {
            textures: (0..columns * rows).into_iter().map(|_| image_assets.add(Image::default())).collect(),
            sprite_dimension,
            columns,
            rows,
        }
    }

    pub fn get_textures<'a>(&'a self) -> impl IntoIterator<Item=Handle<Image>> + 'a {
        self.textures.iter().map(Clone::clone)
    }

    fn create_textures(&mut self, image: &Image, image_assets: &mut Assets<Image>) {
        for y in 0..self.rows {
            for x in 0..self.columns {
                let index = (y * self.columns) + x;
                let new_image = self.create_texture(image, x, y);
                self.textures[index] = image_assets.set(self.textures[index].id, new_image);
            }
        }
    }

    fn create_texture(&self, sheet_image: &Image, column: usize, row: usize) -> Image {
        let size = Extent3d {
            width: self.sprite_dimension.x as u32,
            height: self.sprite_dimension.y as u32,
            depth_or_array_layers: sheet_image.texture_descriptor.size.depth_or_array_layers,
        };
        let dimension = sheet_image.texture_descriptor.dimension;
        let format = sheet_image.texture_descriptor.format;

        let sheet_width = sheet_image.texture_descriptor.size.width as usize * format.pixel_size();
        let width = self.sprite_dimension.x as usize * format.pixel_size();
        let height = self.sprite_dimension.y as usize;
        let data = get_sub_rect(sheet_image.data.as_slice(), sheet_width, column, row, width, height);

        Image::new(
            size,
            dimension,
            data,
            format,
        )
    }
}