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
    #[asset(path = "player/uk_idle.png")]
    pub placeholder: Handle<Image>,
    #[asset(path = "player/fw_idle.png")]
    pub forward_idle: Handle<Image>,
    #[asset(path = "player/bw_idle.png")]
    pub backward_idle: Handle<Image>
}

#[derive(AssetCollection)]
pub struct NPCAssets {
    #[asset(path = "nothing.png")]
    pub unloaded: Handle<Image>,
    #[asset(path = "npc/thomas_kontos/down.png")]
    pub thomas_kontos_face_down: Handle<Image>,
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
    #[asset(path = "nothing.png")]
    pub none: Handle<Image>,
    #[asset(path = "item/demo_axe.png")]
    pub demo_axe: Handle<Image>,
    #[asset(path = "item/placeholder.png")]
    pub demo_rod: Handle<Image>
}

impl ItemAssets {
    pub fn pick_from_item(&self, item: Item) -> Handle<Image> {
        match item {
            Item::None => self.none.clone(),
            Item::DemoAxe => self.demo_axe.clone(),
            Item::DemoRod => self.demo_rod.clone()
        }
    }
}

#[derive(AssetCollection)]
pub struct ObjectAssets {
    #[asset(path = "object/placeholder.png")]
    pub tree: Handle<Image>
}
