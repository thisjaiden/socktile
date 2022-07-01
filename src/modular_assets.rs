use bevy::{
    utils::HashMap,
    reflect::TypeUuid,
    prelude::*,
    asset::{
        AssetLoader,
        LoadContext,
        BoxedFuture,
        LoadedAsset,
        AssetPath
    }
};
use bevy_kira_audio::AudioSource;
use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::consts::{FATAL_ERROR, EMBED_ASSETS};

#[derive(Default)]
pub struct ModularAssetsPlugin;

impl Plugin for ModularAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<ModularAssets>()
            .add_asset_loader(ModularAssetsLoader);
    }
}

#[derive(TypeUuid)]
#[uuid = "8d513cb4-0fa2-4069-b6ad-fb7e8dd37031"]
pub struct ModularAssets {
    // TODO: will contain all assets loaded from modules and json
    audio_samples: Vec<(AudioSampleMetadata, Handle<AudioSource>)>,
    language_keys: HashMap<String, LanguageValue>,
    terrain_data: TerrainData
}

impl ModularAssets {
    pub fn get_audio(&self, name: String) -> Handle<AudioSource> {
        for (meta, handle) in &self.audio_samples {
            if meta.name == name {
                return handle.clone();
            }
        }
        error!("Unable to find an audio sample with the name {}", name);
        panic!("{FATAL_ERROR}");
    }
    pub fn get_tile_rendering(&self, environment: [usize; 9]) -> TerrainRendering {
        // check environment validity
        for tile in environment {
            if tile > self.terrain_data.states.len() {
                error!("Invalid environment passed");
                panic!("{FATAL_ERROR}");
            }
        }
        // check for a uniform environment
        if environment.iter().min() == environment.iter().max() {
            return 
        }
    }
}

pub struct ModularAssetsLoader;

impl AssetLoader for ModularAssetsLoader {
    fn load<'a>(
        &'a self,
        _bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let mut final_out = ModularAssets {
                audio_samples: vec![],
                language_keys: default(),
                terrain_data: default()
            };
            let audio_core: AudioMetadata;
            let lang_core: Value;
            let terrain_core: TerrainData;
            let transition_core: Vec<TerrainTransition>;

            if EMBED_ASSETS {
                audio_core = serde_json::from_slice(include_bytes!("../assets/metadata/audio.json")).unwrap();
                lang_core = serde_json::from_slice(include_bytes!("../assets/lang/en_us.json")).unwrap();
                terrain_core = serde_json::from_slice(include_bytes!("../assets/metadata/terrain.json")).unwrap();
                transition_core = serde_json::from_slice(include_bytes!("../assets/metadata/transitions.json")).unwrap();
            }
            else {
                audio_core = serde_json::from_str(&std::fs::read_to_string("../assets/metadata/audio.json").unwrap()).unwrap();
                lang_core = serde_json::from_str(&std::fs::read_to_string("../assets/lang/en_us.json").unwrap()).unwrap();
                terrain_core = serde_json::from_str(&std::fs::read_to_string("../assets/metadata/terrain.json").unwrap()).unwrap();
                transition_core = serde_json::from_str(&std::fs::read_to_string("../assets/metadata/transitions.json").unwrap()).unwrap();
            }

            let mut dependencies = vec![];

            for sample in audio_core.audio_samples {
                let path: AssetPath = load_context
                    .path()
                    .parent()
                    .unwrap()
                    .join(format!("audio/{}", sample.meta_location))
                    .into();
                final_out.audio_samples.push((sample, load_context.get_handle(path.clone())));
                dependencies.push(path);
            }

            let keys = grab_keys_recursively(String::from("en_us"), lang_core);
            for (key, value) in keys {
                final_out.language_keys.insert(key, value);
            }
            
            let loaded_asset = LoadedAsset::new(final_out);
            load_context.set_default_asset(loaded_asset.with_dependencies(dependencies));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &[];
        EXTENSIONS
    }
}

/// Takes the keys out of a json object and monosizes them into (Key, Value) pairs.
/// Subobjects are appended with a .[object] phrase
fn grab_keys_recursively(current_key: String, current_value: Value) -> Vec<(String, LanguageValue)> {
    let mut returnable = vec![];
    for (key, value) in current_value.as_object().unwrap() {
        if value.is_string() {
            returnable.push((format!("{}.{}", current_key, key), LanguageValue::Value(value.to_string())));
        }
        if value.is_array() {
            let mut smallarray = vec![];
            for element in value.as_array().unwrap() {
                smallarray.push(element.as_str().unwrap().to_string());
            }
            returnable.push((format!("{}.{}", current_key, key), LanguageValue::RandomValue(smallarray)));
        }
        if value.is_object() {
            returnable.append(&mut grab_keys_recursively(format!("{}.{}", current_key, key), value.clone()));
        }
    }
    return returnable;
}

#[derive(Deserialize)]
struct AudioMetadata {
    pub audio_samples: Vec<AudioSampleMetadata>
}

#[derive(Deserialize)]
struct AudioSampleMetadata {
    pub name: String,
    pub meta_location: String
}

enum LanguageValue {
    Value(String),
    RandomValue(Vec<String>)
}

#[derive(Deserialize)]
struct TerrainData {
    minimum_height: usize,
    maximum_height: usize,
    states: Vec<TerrainState>,
    #[serde(skip)]
    transitions: Vec<TerrainTransition>
}

impl Default for TerrainData {
    fn default() -> TerrainData {
        TerrainData {
            minimum_height: 0,
            maximum_height: 0,
            states: vec![],
            transitions: vec![]
        }
    }
}


#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
/// Represents the state of a single tile of terrain.
struct TerrainState {
    name: String,
    meta_location: String,
    approx_color: String,
    walk_sound: String,
    run_sound: String
}

#[derive(Deserialize)]
struct TerrainTransition {
    names: Vec<String>,
    meta_location: String,
}

pub enum TerrainRendering {
    Sprite(Handle<Image>),
    SpriteSheet(Handle<Image>, usize),
    AnimatedSprite(Vec<Handle<Image>>, AnimationInfo),
    AnimatedSpriteSheet(Handle<Image>, AnimationInfo)
}

pub struct AnimationInfo {
    pub num_states: usize,
    pub ticks_between_states: usize,
}

/* 
impl TerrainState {
    pub fn collides(&mut self, player: (f64, f64), offset_x: f64, offset_y: f64) -> bool {
        // TODO: properly define player hitbox beyond arbitrary numbers here
        self.collider_type().does_collide_with((player.0 - 32.0, player.1 - 28.0, 64.0, 64.0), offset_x, offset_y)
    }
    fn collider_type(&mut self) -> ColliderType {
        match self.tileset {
            58 | 85 => {
                match self.tile {
                    0 => ColliderType::TopLeft,
                    1 => ColliderType::Top,
                    2 => ColliderType::TopRight,
                    3 => ColliderType::InverseTopLeft,
                    4 => ColliderType::InverseTopRight,
                    8 => ColliderType::Left,
                    10 => ColliderType::Right,
                    11 => ColliderType::InverseBottomLeft,
                    12 => ColliderType::InverseBottomRight,
                    16 => ColliderType::BottomLeft,
                    17 => ColliderType::Bottom,
                    18 => ColliderType::BottomRight,
                    9 | 19 | 24..=28 | 32 | 34..=36 | 40..=42 => ColliderType::None,
                    invalid_id => {
                        error!("Unknown tile id in generic style tilesheet ({}:{invalid_id})", self.tileset);
                        panic!("{FATAL_ERROR}");
                    }
                }
            }
            invalid_id => {
                error!("Unknown tileset id {invalid_id}");
                panic!("{FATAL_ERROR}");
            }
        }
    }
}

#[derive(Debug)]
pub enum ColliderType {
    // No collider
    None,
    // Thin colliders prevent movement across the respective sides of the tile
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
    InverseTopLeft,
    InverseTopRight,
    InverseBottomLeft,
    InverseBottomRight,
}

impl ColliderType {
    fn collider_dimensions(&mut self) -> &[(f64, f64, f64, f64)] {
        match self {
            Self::None => &[],
            Self::TopLeft => &[(26.0, 0.0, 6.0, 32.0), (32.0, 32.0, 32.0, 6.0)],
            Self::Top => &[(0.0, 32.0, 64.0, 6.0)],
            Self::TopRight => &[(0.0, 32.0, 32.0, 6.0), (32.0, 0.0, 6.0, 32.0)],
            Self::Left => &[(26.0, 0.0, 6.0, 64.0)],
            Self::Right => &[(32.0, 0.0, 6.0, 64.0)],
            Self::BottomLeft => &[(26.0, 32.0, 6.0, 32.0), (32.0, 26.0, 32.0, 6.0)],
            Self::Bottom => &[(0.0, 26.0, 64.0, 6.0)],
            Self::BottomRight => &[(0.0, 26.0, 32.0, 6.0), (32.0, 32.0, 6.0, 32.0)],
            Self::InverseTopLeft => &[(32.0, 0.0, 6.0, 32.0), (32.0, 26.0, 32.0, 6.0)],
            Self::InverseTopRight => &[(0.0, 26.0, 32.0, 6.0), (26.0, 0.0, 6.0, 32.0)],
            Self::InverseBottomLeft => &[(32.0, 32.0, 32.0, 6.0), (32.0, 32.0, 6.0, 32.0)],
            Self::InverseBottomRight => &[(0.0, 32.0, 32.0, 6.0), (26.0, 32.0, 6.0, 32.0)]
        }
    }
    fn cube_colliders(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> bool {
        a.0 < (b.0 + b.2) &&
        (a.0 + a.2) > b.0 &&
        (a.1 + a.3) > b.1 &&
        a.1 < (b.1 + b.3)
    }
    pub fn does_collide_with(&mut self, other: (f64, f64, f64, f64), offset_x: f64, offset_y: f64) -> bool {
        let mut checks = vec![];
        for collider in self.collider_dimensions() {
            checks.push(Self::cube_colliders(
                (
                    collider.0 + offset_x,
                    collider.1 + offset_y,
                    collider.2,
                    collider.3
                ),
                other
            ));
        }
        checks.contains(&true)
    }
}
*/
