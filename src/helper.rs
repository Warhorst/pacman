use bevy::asset::AssetPath;
use bevy::prelude::*;

pub fn load_textures<'a, P: Into<AssetPath<'a>>, I: IntoIterator<Item=P> + 'a>(asset_server: &'a AssetServer, paths: I) -> impl IntoIterator<Item=Handle<Image>> + 'a {
    paths.into_iter().map(|p| asset_server.load(p))
}