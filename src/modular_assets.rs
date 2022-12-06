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
    Everything,
    TopLeftBottomLeft,
    TopLeftBottomRight,
    TopRightBottomLeft,
    TopLeftTopRight,
    FTopBottomLeft,
    FTopBottomRight,
    FLeftBottomRight,
    FRightBottomLeft,
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
    // bool = is dominant (true = yes, false = no)
    // Credit for this being only half the abomination it used to be: Evelyn! <3
    pub fn get_from_environment(environment: Vec<bool>) -> Option<TransitionType> {
        // 0 1 2
        // 3 4 5
        // 6 7 8
        if environment == [false; 9] {
            return Some(TransitionType::Nothing);
        }
        if !environment[4] {
            if environment[1] {
                if environment[3] {
                    // all states with at least a FTopFLeft in them
                    if environment[5] {
                        if environment[7] {
                            return Some(TransitionType::FTopFLeftFBottomFRight);
                        }
                        else {
                            return Some(TransitionType::FTopFLeftFRight);
                        }
                    }
                    else if environment[7] {
                        return Some(TransitionType::FTopFLeftFBottom);
                    }
                    else if environment[8] {
                        return Some(TransitionType::FTopFLeftBottomRight);
                    }
                    else {
                        return Some(TransitionType::FTopFLeft);
                    }
                }
                else if environment[5] {
                    // all states with at least a FTopFRight in them
                    if environment[7] {
                        return Some(TransitionType::FTopFBottomFRight);
                    }
                    else if environment[6] {
                        return Some(TransitionType::FTopFRightBottomLeft);
                    }
                    else {
                        return Some(TransitionType::FTopFRight);
                    }
                }
                else if environment[7] {
                    // all states with at least a FTopFBottom in them
                    return Some(TransitionType::FTopFBottom);
                }
                // all states with at only an FTop and unknown corners
                if environment[6] {
                    if environment[8] {
                        return Some(TransitionType::FTopBottomLeftBottomRight);
                    }
                    else {
                        return Some(TransitionType::FTopBottomLeft);
                    }
                }
                else if environment[8] {
                    return Some(TransitionType::FTopBottomRight);
                }
            }
            else if environment[3] {
                if environment[5] {
                    if environment[7] {
                        return Some(TransitionType::FLeftFBottomFRight);
                    }
                    else {
                        return Some(TransitionType::FLeftFRight);
                    }
                }
                else if environment[7] {
                    if environment[2] {
                        return Some(TransitionType::FLeftFBottomTopRight);
                    }
                    else {
                        return Some(TransitionType::FLeftFBottom);
                    }
                }
                if environment[2] {
                    if environment[8] {
                        return Some(TransitionType::FLeftTopRightBottomRight);
                    }
                    else {
                        return Some(TransitionType::FLeftTopRight);
                    }
                }
                else if environment[8] {
                    return Some(TransitionType::FLeftBottomRight);
                }
            }
            else if environment[5] {
                if environment[7] {
                    if environment[0] {
                        return Some(TransitionType::FBottomFRightTopLeft);
                    }
                    else {
                        return Some(TransitionType::FBottomFRight);
                    }
                }
                if environment[0] {
                    if environment[6] {
                        return Some(TransitionType::FRightTopLeftBottomLeft);
                    }
                    else {
                        return Some(TransitionType::FRightTopLeft);
                    }
                }
                else if environment[6] {
                    return Some(TransitionType::FRightBottomLeft);
                }
            }
            else if environment[7] {
                if environment[0] {
                    if environment[2] {
                        return Some(TransitionType::FBottomTopLeftTopRight);
                    }
                    else {
                        return Some(TransitionType::FBottomTopLeft);
                    }
                }
                else if environment[2] {
                    return Some(TransitionType::FBottomTopRight);
                }
                else {
                    return Some(TransitionType::FBottom);
                }
            }
            if environment[0] {
                if environment[2] {
                    if environment[6] {
                        if environment[8] {
                            return Some(TransitionType::TopLeftTopRightBottomLeftBottomRight);
                        }
                        else {
                            return Some(TransitionType::TopLeftTopRightBottomLeft);
                        }
                    }
                    else if environment[8] {
                        return Some(TransitionType::TopLeftTopRightBottomRight);
                    }
                    else {
                        return Some(TransitionType::TopLeftTopRight);
                    }
                }
                else if environment[6] {
                    if environment[8] {
                        return Some(TransitionType::TopLeftBottomLeftBottomRight);
                    }
                    else {
                        return Some(TransitionType::TopLeftBottomLeft);
                    }
                }
                else if environment[8] {
                    return Some(TransitionType::TopLeftBottomRight);
                }
                else {
                    return Some(TransitionType::TopLeft);
                }
            }
            else if environment[2] {
                // ...
            }
            else if environment[6] {
                // ...
            }
            else if environment[8] {
                // ...
            }
        }
        else {
            // Inverted full
            return Some(TransitionType::Everything);
        }
        panic!();
    }
    
}

// This is gross. There must be a better way to do this (I know there is)
// But I don't know how I would do it and I don't care enough. It's *fine*.
//
// Further note: the solution is a macro, which I'm too lazy to do.
// as past me said, "It's *fine*."
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
    if let Some(value) = styles.bt {
        output.push((TransitionType::FBottom, value));
    }
    if let Some(value) = styles.btri {
        output.push((TransitionType::FBottomFRight, value));
    }
    if let Some(value) = styles.btritl {
        output.push((TransitionType::FBottomFRightTopLeft, value));
    }
    if let Some(value) = styles.bttl {
        output.push((TransitionType::FBottomTopLeft, value));
    }
    if let Some(value) = styles.bttltr {
        output.push((TransitionType::FBottomTopLeftTopRight, value));
    }
    if let Some(value) = styles.bttr {
        output.push((TransitionType::FBottomTopRight, value));
    }
    if let Some(value) = styles.lf {
        output.push((TransitionType::FLeft, value));
    }
    if let Some(value) = styles.lfbr {
        output.push((TransitionType::FLeftBottomRight, value));
    }
    if let Some(value) = styles.lfbt {
        output.push((TransitionType::FLeftFBottom, value));
    }
    if let Some(value) = styles.lfbtri {
        output.push((TransitionType::FLeftFBottomFRight, value));
    }
    if let Some(value) = styles.lfbttr {
        output.push((TransitionType::FLeftFBottomTopRight, value));
    }
    if let Some(value) = styles.lfri {
        output.push((TransitionType::FLeftFRight, value));
    }
    if let Some(value) = styles.lftr {
        output.push((TransitionType::FLeftTopRight, value));
    }
    if let Some(value) = styles.lftrbr {
        output.push((TransitionType::FLeftTopRightBottomRight, value));
    }
    if let Some(value) = styles.nz {
        output.push((TransitionType::Nothing, value));
    }
    if let Some(value) = styles.ri {
        output.push((TransitionType::FRight, value));
    }
    if let Some(value) = styles.ribl {
        output.push((TransitionType::FRightBottomLeft, value));
    }
    if let Some(value) = styles.ritl {
        output.push((TransitionType::FRightTopLeft, value));
    }
    if let Some(value) = styles.ritlbl {
        output.push((TransitionType::FRightTopLeftBottomLeft, value));
    }
    if let Some(value) = styles.tl {
        output.push((TransitionType::TopLeft, value));
    }
    if let Some(value) = styles.tlbl {
        output.push((TransitionType::TopLeftBottomLeft, value));
    }
    if let Some(value) = styles.tlblbr {
        output.push((TransitionType::TopLeftBottomLeftBottomRight, value));
    }
    if let Some(value) = styles.tlbr {
        output.push((TransitionType::TopLeftBottomRight, value));
    }
    if let Some(value) = styles.tltr {
        output.push((TransitionType::TopLeftTopRight, value));
    }
    if let Some(value) = styles.tltrbl {
        output.push((TransitionType::TopLeftTopRightBottomLeft, value));
    }
    if let Some(value) = styles.tltrblbr {
        output.push((TransitionType::TopLeftTopRightBottomLeftBottomRight, value));
    }
    if let Some(value) = styles.tltrbr {
        output.push((TransitionType::TopLeftTopRightBottomRight, value));
    }
    if let Some(value) = styles.tp {
        output.push((TransitionType::FTop, value));
    }
    if let Some(value) = styles.tpbl {
        output.push((TransitionType::FTopBottomLeft, value));
    }
    if let Some(value) = styles.tpblbr {
        output.push((TransitionType::FTopBottomLeftBottomRight, value));
    }
    if let Some(value) = styles.tpbr {
        output.push((TransitionType::FTopBottomRight, value));
    }
    if let Some(value) = styles.tpbt {
        output.push((TransitionType::FTopFBottom, value));
    }
    if let Some(value) = styles.tpbtri {
        output.push((TransitionType::FTopFBottomFRight, value));
    }
    if let Some(value) = styles.tplf {
        output.push((TransitionType::FTopFLeft, value));
    }
    if let Some(value) = styles.tplfbr {
        output.push((TransitionType::FTopFLeftBottomRight, value));
    }
    if let Some(value) = styles.tplfbt {
        output.push((TransitionType::FTopFLeftFBottom, value));
    }
    if let Some(value) = styles.tplfbtri {
        output.push((TransitionType::FTopFLeftFBottomFRight, value));
    }
    if let Some(value) = styles.tplfri {
        output.push((TransitionType::FTopFLeftFRight, value));
    }
    if let Some(value) = styles.tpri {
        output.push((TransitionType::FTopFRight, value));
    }
    if let Some(value) = styles.tpribl {
        output.push((TransitionType::FTopFRightBottomLeft, value));
    }
    if let Some(value) = styles.tr {
        output.push((TransitionType::TopRight, value));
    }
    if let Some(value) = styles.trbl {
        output.push((TransitionType::TopRightBottomLeft, value));
    }
    if let Some(value) = styles.trblbr {
        output.push((TransitionType::TopRightBottomLeftBottomRight, value));
    }
    if let Some(value) = styles.trbr {
        output.push((TransitionType::TopRightBottomRight, value));
    }
    output
}


