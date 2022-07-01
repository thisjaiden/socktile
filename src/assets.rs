use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

use crate::{shared::player::Item, modular_assets};

#[derive(AssetCollection)]
pub struct CoreAssets {
    #[asset(path = "metadata")]
    pub core: Handle<modular_assets::ModularAssets>,
    #[asset(path = "core/title_screen.png")]
    pub title_screen: Handle<Image>,
    #[asset(path = "core/create_user.png")]
    pub create_user: Handle<Image>,
    #[asset(path = "core/create_world.png")]
    pub create_world: Handle<Image>,
    #[asset(path = "core/join_world.png")]
    pub join_world: Handle<Image>,
    #[asset(path = "core/video_settings.png")]
    pub video_settings: Handle<Image>,
    #[asset(path = "core/offline.png")]
    pub offline_no_support: Handle<Image>,
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
    /// Represents a player that has not yet moved or had any animation occur.
    #[asset(path = "player/default.png")]
    pub not_animated: Handle<Image>,
    #[asset(path = "player/idle/0.png")]
    pub idle0: Handle<Image>,
    #[asset(path = "player/idle/1.png")]
    pub idle1: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct NPCAssets {
    #[asset(path = "npc/thomas_kontos/idle/0.png")]
    pub not_animated: Handle<Image>,
    #[asset(path = "npc/thomas_kontos/idle/0.png")]
    pub idle0: Handle<Image>,
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
