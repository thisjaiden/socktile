use crate::prelude::*;
use bevy_asset_loader::prelude::*;

pub mod animated_sprite;
pub mod audio;
pub mod language;
pub mod tiles;

#[derive(AssetCollection, Resource)]
pub struct CoreAssets {
    #[asset(path = "lang/en_us.ljson")]
    pub lang: Handle<LanguageKeys>,
    #[asset(path = "metadata/audio.sjson")]
    pub audio: Handle<AudioSamples>,
    #[asset(path = "metadata/terrain.tjson")]
    pub tiles: Handle<TileTypeConfig>,
    #[asset(path = "metadata/transitions.ujson")]
    pub transitions: Handle<TileTransitionMasterConfig>,
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
    #[asset(path = "core/nothing.png")]
    pub blank: Handle<Image>,
}

#[derive(AssetCollection, Resource, Clone)]
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
    pub kreative_square: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AnimatorAssets {
    /// Represents a player that has not yet moved or had any animation occur.
    #[asset(path = "player/default.png")]
    pub not_animated: Handle<Image>,
    #[asset(path = "player/idle/0.png")]
    pub idle0: Handle<Image>,
    #[asset(path = "player/idle/1.png")]
    pub idle1: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct NPCAssets {
    #[asset(path = "ui/npc_popup/grow.ajson")]
    pub popup_grow: Handle<AnimatedSprite>,
    #[asset(path = "ui/npc_popup/shrink.ajson")]
    pub popup_shrink: Handle<AnimatedSprite>,
    #[asset(path = "npc/thomas_kontos/idle/0.png")]
    pub not_animated: Handle<Image>,
    #[asset(path = "npc/thomas_kontos/idle/0.png")]
    pub idle0: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct UIAssets {
    #[asset(path = "ui/slot.png")]
    pub slot: Handle<Image>,
    #[asset(path = "ui/selected.png")]
    pub selected: Handle<Image>,
    #[asset(path = "ui/blueprint_selector.png")]
    pub blueprint: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct ItemAssets {
    #[asset(path = "nothing.png")]
    pub none: Handle<Image>,
    #[asset(path = "item/makeshift_axe.png")]
    pub makeshift_axe: Handle<Image>,
    // axe
    // reinforced axe
    // opalescent axe
    #[asset(path = "item/placeholder.png")]
    pub makeshift_fishing_rod: Handle<Image>,
    #[asset(path = "item/blueprint.png")]
    pub blueprint: Handle<Image>,
    #[asset(path = "item/wood.png")]
    pub wood: Handle<Image>,
}

impl ItemAssets {
    pub fn pick_from_item(&self, item: Option<Item>) -> Handle<Image> {
        if item.is_none() {
            return self.none.clone();
        }
        match item.unwrap() {
            Item::MakeshiftAxe => self.makeshift_axe.clone(),
            Item::MakeshiftFishingRod => self.makeshift_fishing_rod.clone(),
            Item::Blueprint => self.blueprint.clone(),
            Item::Wood => self.wood.clone(),
        }
    }
}

#[derive(AssetCollection, Resource)]
pub struct ObjectAssets {
    #[asset(path = "object/tree_ly.png")]
    pub tree: Handle<Image>,
}
