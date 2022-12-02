use crate::prelude::{*, tiles::{TileTypeConfig, TileTypeConfigLoader, TileTransitionMasterConfig, TileTransitionMasterConfigLoader, TileTransitionConfig, TileTransitionConfigLoader, Variant}};

use crate::language::LanguageKeysLoader;
use crate::audio::AudioSamplesLoader;

#[derive(Default)]
pub struct ModularAssetsPlugin;

impl Plugin for ModularAssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_asset::<LanguageKeys>()
            .add_asset_loader(LanguageKeysLoader)
            .add_asset::<AudioSamples>()
            .add_asset_loader(AudioSamplesLoader)
            .add_asset::<TileTypeConfig>()
            .add_asset_loader(TileTypeConfigLoader)
            .add_asset::<TileTransitionMasterConfig>()
            .add_asset_loader(TileTransitionMasterConfigLoader)
            .add_asset::<TileTransitionConfig>()
            .add_asset_loader(TileTransitionConfigLoader);
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum TransitionType {
    Center,
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    InvertedUpLeft,
    InvertedUpRight,
    InvertedDownLeft,
    InvertedDownRight
}

impl TransitionType {
    pub fn collides(&self, player_location: (f32, f32), offset_x: f32, offset_y: f32) -> bool {
        for collider in self.collider_dimensions() {
            if TransitionType::cube_colliders(
                (
                    collider.0 + offset_x,
                    collider.1 + offset_y,
                    collider.2,
                    collider.3,
                ),
                (
                    player_location.0 - 32.0,
                    player_location.1 - 28.0,
                    PLAYER_HITBOX.0,
                    PLAYER_HITBOX.1
                )
            ) {
                return true;
            }
        }
        false
    }
    pub fn collider_dimensions(&self) -> &[(f32, f32, f32, f32)] {
        match self {
            Self::Center => &[],
            Self::UpLeft => &[(26.0, 0.0, 6.0, 32.0), (32.0, 32.0, 32.0, 6.0)],
            Self::Up => &[(0.0, 32.0, 64.0, 6.0)],
            Self::UpRight => &[(0.0, 32.0, 32.0, 6.0), (32.0, 0.0, 6.0, 32.0)],
            Self::Left => &[(26.0, 0.0, 6.0, 64.0)],
            Self::Right => &[(32.0, 0.0, 6.0, 64.0)],
            Self::DownLeft => &[(26.0, 32.0, 6.0, 32.0), (32.0, 26.0, 32.0, 6.0)],
            Self::Down => &[(0.0, 26.0, 64.0, 6.0)],
            Self::DownRight => &[(0.0, 26.0, 32.0, 6.0), (32.0, 32.0, 6.0, 32.0)],
            Self::InvertedUpLeft => &[(32.0, 0.0, 6.0, 32.0), (32.0, 26.0, 32.0, 6.0)],
            Self::InvertedUpRight => &[(0.0, 26.0, 32.0, 6.0), (26.0, 0.0, 6.0, 32.0)],
            Self::InvertedDownLeft => &[(32.0, 32.0, 32.0, 6.0), (32.0, 32.0, 6.0, 32.0)],
            Self::InvertedDownRight => &[(0.0, 32.0, 32.0, 6.0), (26.0, 32.0, 6.0, 32.0)]
        }
    }
    fn cube_colliders(a: (f32, f32, f32, f32), b: (f32, f32, f32, f32)) -> bool {
        a.0 < (b.0 + b.2) &&
        (a.0 + a.2) > b.0 &&
        (a.1 + a.3) > b.1 &&
        a.1 < (b.1 + b.3)
    }
    
}

// This is gross. There must be a better way to do this (I know there is)
// But I don't know how I would do it and I don't care enough. It's *fine*.
pub fn conjoin_styles(styles: Variant) -> Vec<(TransitionType, Vec<usize>)> {
    let mut output = vec![];
    if let Some(value) = styles.center {
        output.push((TransitionType::Center, value));
    }
    if let Some(value) = styles.down {
        output.push((TransitionType::Down, value));
    }
    if let Some(value) = styles.left {
        output.push((TransitionType::Left, value));
    }
    if let Some(value) = styles.right {
        output.push((TransitionType::Right, value));
    }
    if let Some(value) = styles.up {
        output.push((TransitionType::Up, value));
    }
    if let Some(value) = styles.up_left {
        output.push((TransitionType::UpLeft, value));
    }
    if let Some(value) = styles.up_right {
        output.push((TransitionType::UpRight, value));
    }
    if let Some(value) = styles.down_left {
        output.push((TransitionType::DownLeft, value));
    }
    if let Some(value) = styles.down_right {
        output.push((TransitionType::DownRight, value));
    }
    if let Some(value) = styles.inverted_up_left {
        output.push((TransitionType::InvertedUpLeft, value));
    }
    if let Some(value) = styles.inverted_up_right {
        output.push((TransitionType::InvertedUpRight, value));
    }
    if let Some(value) = styles.inverted_down_left {
        output.push((TransitionType::InvertedDownLeft, value));
    }
    if let Some(value) = styles.inverted_down_right {
        output.push((TransitionType::InvertedDownRight, value));
    }
    output
}


