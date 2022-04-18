use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

use crate::{ldtk, shared::player::Item};

#[derive(AssetCollection)]
pub struct MapAssets {
    #[asset(path = "core.ldtk")]
    pub core: Handle<ldtk::LDtkMap>,
}

#[derive(AssetCollection, Clone)]
pub struct FontAssets {
    #[asset(path = "font/apple_tea.ttf")]
    pub apple_tea: Handle<Font>,
    #[asset(path = "font/simvoni/regular.ttf")]
    pub simvoni: Handle<Font>,
    #[asset(path = "font/simvoni/italic.ttf")]
    _simvoni_italic: Handle<Font>,
    #[asset(path = "font/simvoni/bold.ttf")]
    pub simvoni_bold: Handle<Font>,
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
    #[asset(path = "player/up.png")]
    pub face_up: Handle<Image>,
    #[asset(path = "player/up_right.png")]
    pub face_up_right: Handle<Image>,
    #[asset(path = "player/up_left.png")]
    pub face_up_left: Handle<Image>,
    #[asset(path = "player/down.png")]
    pub face_down: Handle<Image>,
    #[asset(path = "player/down_left.png")]
    pub face_down_left: Handle<Image>,
    #[asset(path = "player/down_right.png")]
    pub face_down_right: Handle<Image>,
    #[asset(path = "player/left.png")]
    pub face_left: Handle<Image>,
    #[asset(path = "player/right.png")]
    pub face_right: Handle<Image>
}

#[derive(AssetCollection)]
pub struct UIAssets {
    #[asset(path = "ui/slot.png")]
    pub slot: Handle<Image>,
    #[asset(path = "ui/selected.png")]
    pub selected: Handle<Image>
}

#[derive(AssetCollection)]
pub struct ItemAssets {
    #[asset(path = "item/placeholder.png")]
    pub demo_axe: Handle<Image>
}

impl ItemAssets {
    pub fn pick_from_item(&self, item: Item) -> Handle<Image> {
        match item {
            Item::None => panic!("You can't pick from no item!"),
            Item::DemoAxe => self.demo_axe.clone()
        }
    }
}

#[derive(AssetCollection)]
pub struct ObjectAssets {
    #[asset(path = "object/placeholder.png")]
    pub tree: Handle<Image>
}
