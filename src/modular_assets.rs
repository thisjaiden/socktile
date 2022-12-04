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
    TopLeft,
    TopRight,
    FTop,
    FRight,
    FBottom,
    BottomLeft,
    BottomRight,
    FLeft,
    Nothing,
    TopLeftBottomLeft,
    TopLeftBottomRight,
    TopRightBottomLeft,
    TopLeftTopRight,
    FTopBottomLeft,
    FTopBottomRight,
    FLeftBottomRight,
    FTopFLeft,
    FTopFRight,
    FLeftFBottom,
    FTopFBottom,
    FLeftFRight,
    FBottomFRight,
    FBottomTopLeft,
    FBottomTopRight,
    FLeftTopRight,
    FRightTopLeft,
    BottomLeftBottomRight,
    TopRightBottomRight,
    FTopBottomLeftBottomRight,
    FBottomTopLeftTopRight,
    FTopFLeftBottomRight,
    FTopFLeftFRight,
    FTopFLeftFBottom,
    FTopFRightBottomLeft,
    FLeftFBottomTopRight,
    FLeftFBottomFRight,
    FTopFBottomFRight,
    FBottomFRightTopLeft,
    TopLeftBottomLeftBottomRight,
    TopRightBottomLeftBottomRight,
    FLeftTopRightBottomRight,
    FRightTopLeftBottomLeft,
    TopLeftTopRightBottomLeft,
    TopLeftTopRightBottomRight,
    FTopFLeftFBottomFRight,
    TopLeftTopRightBottomLeftBottomRight
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
            Self::Nothing => &[],
            _ => todo!()
        }
    }
    fn cube_colliders(a: (f32, f32, f32, f32), b: (f32, f32, f32, f32)) -> bool {
        a.0 < (b.0 + b.2) &&
        (a.0 + a.2) > b.0 &&
        (a.1 + a.3) > b.1 &&
        a.1 < (b.1 + b.3)
    }
    fn get_from_environment(environment: [usize; 9]) -> Option<TransitionType> {
        todo!()
    }
    
}

// This is gross. There must be a better way to do this (I know there is)
// But I don't know how I would do it and I don't care enough. It's *fine*.
pub fn conjoin_styles(styles: Variant) -> Vec<(TransitionType, Vec<usize>)> {
    let mut output = vec![];
    if let Some(value) = styles.bl {
        output.push((TransitionType::BottomLeft, value));
    }
    if let Some(value) = styles.blbr {
        output.push((TransitionType::BottomLeftBottomRight, value));
    }
    if let Some(value) = styles.br {
        output.push((TransitionType::BottomRight, value));
    }
    // TODO: Rest of variants
    output
}


