use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

use crate::ldtk;

#[derive(AssetCollection)]
pub struct MapAssets {
    #[asset(path = "core.ldtk")]
    pub core: Handle<ldtk::LDtkMap>,
}

#[derive(AssetCollection, Clone)]
pub struct FontAssets {
    #[asset(path = "font/apple_tea.ttf")]
    _apple_tea: Handle<Font>,
    #[asset(path = "font/simvoni/regular.ttf")]
    pub simvoni: Handle<Font>,
    #[asset(path = "font/simvoni/italic.ttf")]
    _simvoni_italic: Handle<Font>,
    #[asset(path = "font/simvoni/bold.ttf")]
    _simvoni_bold: Handle<Font>,
    #[asset(path = "font/simvoni/bolditalic.ttf")]
    _simvoni_bold_italic: Handle<Font>,
    /// WARNING: DEPRECATED FONT
    #[asset(path = "font/kreative_square.ttf")]
    pub kreative_square: Handle<Font>
}

#[derive(AssetCollection)]
pub struct AnimatorAssets {
    #[asset(path = "player/placeholder.png")]
    pub placeholder: Handle<Image>,
    #[asset(path = "player/placeholder.png")]
    pub face_up: Handle<Image>,
    #[asset(path = "player/placeholder.png")]
    pub face_up_right: Handle<Image>,
    #[asset(path = "player/placeholder.png")]
    pub face_up_left: Handle<Image>,
    #[asset(path = "player/placeholder.png")]
    pub face_down: Handle<Image>,
    #[asset(path = "player/placeholder.png")]
    pub face_down_left: Handle<Image>,
    #[asset(path = "player/placeholder.png")]
    pub face_down_right: Handle<Image>,
    #[asset(path = "player/placeholder.png")]
    pub face_left: Handle<Image>,
    #[asset(path = "player/placeholder.png")]
    pub face_right: Handle<Image>

}
