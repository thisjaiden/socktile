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
    pub fn collides(&self, player_location: (f64, f64), offset_x: f64, offset_y: f64) -> bool {
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
    pub fn collider_dimensions(&self) -> &[(f64, f64, f64, f64)] {
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
    fn cube_colliders(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> bool {
        a.0 < (b.0 + b.2) &&
        (a.0 + a.2) > b.0 &&
        (a.1 + a.3) > b.1 &&
        a.1 < (b.1 + b.3)
    }
    pub fn get_from_environment(environment: [usize; 9]) -> Option<(Self, usize, usize)> {
        // check for a uniform environment
        if environment.iter().min() == environment.iter().max() {
            // found!
            return Some((TransitionType::Center, environment[0], environment[0]));
        }
        // check for a environment with 2 distinct types
        let mut environment_types = vec![];
        for tile in environment {
            if !environment_types.contains(&tile) {
                environment_types.push(tile);
            }
        }
        if environment_types.len() == 2 {
            // found!
            if  environment[0] != environment[4] && environment[1] != environment[4] &&
                environment[2] != environment[4] && environment[3] == environment[4] &&
                environment[5] == environment[4] && environment[6] == environment[4] &&
                environment[7] == environment[4] && environment[8] == environment[4] {
                // AAA
                // BBB
                // BBB
                return Some((TransitionType::Up, environment[4], environment[0]));
            }
            if  environment[0] != environment[4] && environment[1] == environment[4] &&
                environment[2] == environment[4] && environment[3] != environment[4] &&
                environment[5] == environment[4] && environment[6] != environment[4] &&
                environment[7] != environment[4] && environment[8] != environment[4] {
                // ABB
                // ABB
                // AAA
                return Some((TransitionType::DownLeft, environment[4], environment[0]));
            }
            if  environment[0] == environment[4] && environment[1] == environment[4] &&
                environment[2] == environment[4] && environment[3] == environment[4] &&
                environment[5] == environment[4] && environment[6] != environment[4] &&
                environment[7] != environment[4] && environment[8] != environment[4] {
                // BBB
                // BBB
                // AAA
                return Some((TransitionType::Down, environment[4], environment[6]));
            }
            if  environment[0] == environment[4] && environment[1] == environment[4] &&
                environment[2] == environment[4] && environment[3] == environment[4] &&
                environment[5] == environment[4] && environment[6] == environment[4] &&
                environment[7] != environment[4] && environment[8] != environment[4] {
                // BBB
                // BBB
                // BAA
                return Some((TransitionType::Down, environment[4], environment[7]));
            }
            if  environment[0] == environment[4] && environment[1] == environment[4] &&
                environment[2] != environment[4] && environment[3] == environment[4] &&
                environment[5] != environment[4] && environment[6] != environment[4] &&
                environment[7] != environment[4] && environment[8] != environment[4] {
                // BBA
                // BBA
                // AAA
                return Some((TransitionType::DownRight, environment[4], environment[2]));
            }
            if  environment[0] == environment[4] && environment[1] == environment[4] &&
                environment[2] != environment[4] && environment[3] == environment[4] &&
                environment[5] != environment[4] && environment[6] == environment[4] &&
                environment[7] == environment[4] && environment[8] != environment[4] {
                // BBA
                // BBA
                // BBA
                return Some((TransitionType::Right, environment[4], environment[2]));
            }
            if  environment[0] != environment[4] && environment[1] == environment[4] &&
                environment[2] == environment[4] && environment[3] == environment[4] &&
                environment[5] == environment[4] && environment[6] == environment[4] &&
                environment[7] == environment[4] && environment[8] == environment[4] {
                // ABB
                // BBB
                // BBB
                return Some((TransitionType::InvertedDownRight, environment[4], environment[0]));
            }
            if  environment[0] == environment[4] && environment[1] == environment[4] &&
                environment[2] == environment[4] && environment[3] == environment[4] &&
                environment[5] != environment[4] && environment[6] == environment[4] &&
                environment[7] == environment[4] && environment[8] != environment[4] {
                // BBB
                // BBA
                // BBA
                return Some((TransitionType::Right, environment[4], environment[5]));
            }
            if  environment[0] != environment[4] && environment[1] == environment[4] &&
                environment[2] == environment[4] && environment[3] != environment[4] &&
                environment[5] == environment[4] && environment[6] == environment[4] &&
                environment[7] == environment[4] && environment[8] == environment[4] {
                // ABB
                // ABB
                // BBB
                return Some((TransitionType::Left, environment[4], environment[0]));
            }
            if  environment[0] != environment[4] && environment[1] == environment[4] &&
                environment[2] == environment[4] && environment[3] != environment[4] &&
                environment[5] == environment[4] && environment[6] != environment[4] &&
                environment[7] == environment[4] && environment[8] == environment[4] {
                // ABB
                // ABB
                // ABB
                return Some((TransitionType::Left, environment[4], environment[0]));
            }
            if  environment[0] == environment[4] && environment[1] == environment[4] &&
                environment[2] == environment[4] && environment[3] == environment[4] &&
                environment[5] == environment[4] && environment[6] == environment[4] &&
                environment[7] == environment[4] && environment[8] != environment[4] {
                // BBB
                // BBB
                // BBA
                return Some((TransitionType::InvertedUpLeft, environment[4], environment[8]));
            }
            if  environment[0] != environment[4] && environment[1] != environment[4] &&
                environment[2] == environment[4] && environment[3] == environment[4] &&
                environment[5] == environment[4] && environment[6] == environment[4] &&
                environment[7] == environment[4] && environment[8] == environment[4] {
                // AAB
                // BBB
                // BBB
                return Some((TransitionType::Up, environment[4], environment[0]));
            }
            if  environment[0] != environment[4] && environment[1] != environment[4] &&
                environment[2] != environment[4] && environment[3] != environment[4] &&
                environment[5] == environment[4] && environment[6] != environment[4] &&
                environment[7] == environment[4] && environment[8] == environment[4] {
                // AAA
                // ABB
                // ABB
                return Some((TransitionType::UpLeft, environment[4], environment[0]));
            }
            if  environment[0] == environment[4] && environment[1] == environment[4] &&
                environment[2] != environment[4] && environment[3] == environment[4] &&
                environment[5] == environment[4] && environment[6] == environment[4] &&
                environment[7] == environment[4] && environment[8] == environment[4] {
                // BBA
                // BBB
                // BBB
                return Some((TransitionType::InvertedDownLeft, environment[4], environment[2]));
            }
            if  environment[0] == environment[4] && environment[1] != environment[4] &&
                environment[2] != environment[4] && environment[3] == environment[4] &&
                environment[5] == environment[4] && environment[6] == environment[4] &&
                environment[7] == environment[4] && environment[8] == environment[4] {
                // BAA
                // BBB
                // BBB
                return Some((TransitionType::Up, environment[4], environment[1]));
            }
            if  environment[0] == environment[4] && environment[1] == environment[4] &&
                environment[2] != environment[4] && environment[3] == environment[4] &&
                environment[5] != environment[4] && environment[6] == environment[4] &&
                environment[7] == environment[4] && environment[8] == environment[4] {
                // BBA
                // BBA
                // BBB
                return Some((TransitionType::Right, environment[4], environment[2]));
            }
            if  environment[0] != environment[4] && environment[1] != environment[4] &&
                environment[2] != environment[4] && environment[3] == environment[4] &&
                environment[5] != environment[4] && environment[6] == environment[4] &&
                environment[7] == environment[4] && environment[8] != environment[4] {
                // AAA
                // BBA
                // BBA
                return Some((TransitionType::UpRight, environment[4], environment[0]));
            }
            if  environment[0] == environment[4] && environment[1] == environment[4] &&
                environment[2] == environment[4] && environment[3] != environment[4] &&
                environment[5] == environment[4] && environment[6] != environment[4] &&
                environment[7] == environment[4] && environment[8] == environment[4] {
                // BBB
                // ABB
                // ABB
                return Some((TransitionType::Left, environment[4], environment[3]));
            }
            if  environment[0] == environment[4] && environment[1] == environment[4] &&
                environment[2] == environment[4] && environment[3] == environment[4] &&
                environment[5] == environment[4] && environment[6] != environment[4] &&
                environment[7] == environment[4] && environment[8] == environment[4] {
                // BBB
                // BBB
                // ABB
                return Some((TransitionType::InvertedUpRight, environment[4], environment[6]));
            }
            if  environment[0] == environment[4] && environment[1] == environment[4] &&
                environment[2] == environment[4] && environment[3] == environment[4] &&
                environment[5] == environment[4] && environment[6] != environment[4] &&
                environment[7] != environment[4] && environment[8] == environment[4] {
                // BBB
                // BBB
                // AAB
                return Some((TransitionType::Down, environment[4], environment[6]));
            }
            if  environment[0] == environment[4] && environment[1] != environment[4] &&
                environment[2] == environment[4] && environment[3] == environment[4] &&
                environment[5] == environment[4] && environment[6] == environment[4] &&
                environment[7] == environment[4] && environment[8] == environment[4] {
                // BAB
                // BBB
                // BBB
                return Some((TransitionType::Up, environment[4], environment[1]));
            }
            if  environment[0] == environment[4] && environment[1] == environment[4] &&
                environment[2] == environment[4] && environment[3] == environment[4] &&
                environment[5] != environment[4] && environment[6] == environment[4] &&
                environment[7] == environment[4] && environment[8] == environment[4] {
                // BBB
                // BBA
                // BBB
                return Some((TransitionType::Right, environment[4], environment[5]));
            }
            if  environment[0] != environment[4] && environment[1] != environment[4] &&
                environment[2] != environment[4] && environment[3] != environment[4] &&
                environment[5] != environment[4] && environment[6] != environment[4] &&
                environment[7] != environment[4] && environment[8] != environment[4] {
                // AAA
                // ABA
                // AAA
                return Some((TransitionType::Center, environment[4], environment[0]));
            }
            if  environment[0] == environment[4] && environment[1] == environment[4] &&
                environment[2] == environment[4] && environment[3] != environment[4] &&
                environment[5] == environment[4] && environment[6] == environment[4] &&
                environment[7] == environment[4] && environment[8] == environment[4] {
                // BBB
                // ABB
                // BBB
                return Some((TransitionType::Left, environment[4], environment[3]));
            }
            if  environment[0] == environment[4] && environment[1] == environment[4] &&
                environment[2] == environment[4] && environment[3] == environment[4] &&
                environment[5] == environment[4] && environment[6] == environment[4] &&
                environment[7] != environment[4] && environment[8] == environment[4] {
                // BBB
                // BBB
                // BAB
                return Some((TransitionType::Down, environment[4], environment[7]));
            }
            if  environment[0] != environment[4] && environment[1] == environment[4] &&
                environment[2] != environment[4] && environment[3] != environment[4] &&
                environment[5] != environment[4] && environment[6] != environment[4] &&
                environment[7] != environment[4] && environment[8] != environment[4] {
                // ABA
                // ABA
                // AAA
                return Some((TransitionType::Center, environment[4], environment[0]));
            }
            if  environment[0] != environment[4] && environment[1] != environment[4] &&
                environment[2] != environment[4] && environment[3] != environment[4] &&
                environment[5] != environment[4] && environment[6] != environment[4] &&
                environment[7] == environment[4] && environment[8] != environment[4] {
                // AAA
                // ABA
                // ABA
                return Some((TransitionType::Center, environment[4], environment[0]));
            }
            if  environment[0] == environment[4] && environment[1] == environment[4] &&
                environment[2] == environment[4] && environment[3] == environment[4] &&
                environment[5] != environment[4] && environment[6] != environment[4] &&
                environment[7] != environment[4] && environment[8] != environment[4] {
                // BBB
                // BBA
                // AAA
                return Some((TransitionType::DownRight, environment[4], environment[5]));
            }
            if  environment[0] != environment[4] && environment[1] == environment[4] &&
                environment[2] == environment[4] && environment[3] != environment[4] &&
                environment[5] == environment[4] && environment[6] != environment[4] &&
                environment[7] != environment[4] && environment[8] == environment[4] {
                // ABB
                // ABB
                // AAB
                return Some((TransitionType::DownLeft, environment[4], environment[0]));
            }
            if  environment[0] == environment[4] && environment[1] == environment[4] &&
                environment[2] != environment[4] && environment[3] == environment[4] &&
                environment[5] != environment[4] && environment[6] == environment[4] &&
                environment[7] != environment[4] && environment[8] != environment[4] {
                // BBA
                // BBA
                // BAA
                return Some((TransitionType::DownRight, environment[4], environment[2]));
            }
            if  environment[0] != environment[4] && environment[1] != environment[4] &&
                environment[2] != environment[4] && environment[3] != environment[4] &&
                environment[5] == environment[4] && environment[6] == environment[4] &&
                environment[7] == environment[4] && environment[8] == environment[4] {
                // AAA
                // ABB
                // BBB
                return Some((TransitionType::UpLeft, environment[4], environment[0]));
            }
            if  environment[0] != environment[4] && environment[1] != environment[4] &&
                environment[2] != environment[4] && environment[3] == environment[4] &&
                environment[5] != environment[4] && environment[6] == environment[4] &&
                environment[7] == environment[4] && environment[8] == environment[4] {
                // AAA
                // BBA
                // BBB
                return Some((TransitionType::UpRight, environment[4], environment[0]));
            }
            if  environment[0] == environment[4] && environment[1] != environment[4] &&
                environment[2] != environment[4] && environment[3] == environment[4] &&
                environment[5] != environment[4] && environment[6] == environment[4] &&
                environment[7] == environment[4] && environment[8] != environment[4] {
                // BAA
                // BBA
                // BBA
                return Some((TransitionType::UpRight, environment[4], environment[1]));
            }
            warn!("Environment not handled: {:?}", environment);
        }
        // fallback to default
        None
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


