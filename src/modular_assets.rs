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

use crate::consts::{FATAL_ERROR, EMBED_ASSETS, PLAYER_HITBOX};

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
        error!("Unable to find an audio sample with the name '{}'", name);
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
            todo!()
        }
        todo!()
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
            let terrain_core: TerrainDataJSON;
            let transition_core: Vec<TerrainTransitionJSON>;

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

            // audio dependencies
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

            // terrain metadata
            final_out.terrain_data.maximum_height = terrain_core.maximum_height;
            final_out.terrain_data.minimum_height = terrain_core.minimum_height;
            // terrain definitions
            // TODO ^^^
            // terrain transitions
            // For each terrain transition file,
            for transition in transition_core {
                // get the file contents
                info!("Reading ./assets/terrain/{}", transition.meta_location);
                let meta: TerrainRenderingJSON = serde_json::from_str(&std::fs::read_to_string(format!("./assets/terrain/{}", transition.meta_location)).unwrap()).unwrap();
                
                let mut definitions: Vec<ImageDefinition> = vec![];
                // for every image declaration
                for file in meta.files {
                    // load the file
                    let path: AssetPath = load_context
                        .path()
                        .parent()
                        .unwrap()
                        .join(format!("terrain/{}", file.location))
                        .into();
                    // save it to our image definitions
                    if file.width == 1 && file.height == 1 {
                        definitions.push(ImageDefinition::Sprite(load_context.get_handle(path.clone())));
                    }
                    else {
                        definitions.push(ImageDefinition::SpriteSheet(load_context.get_handle(path.clone()), (file.width, file.height)));
                    }
                    // request it to be loaded by bevy
                    dependencies.push(path);
                }
                
                let mut transitions: HashMap<[String; 2], HashMap<TransitionType, Vec<TerrainRendering>>> = default();
                // for every transition declaration
                for variant in meta.variants {
                    let vec_styles = conjoin_styles(variant.clone());
                    let has_value = transitions.contains_key(
                        &[transition.names[0].clone(), transition.names[1].clone()]
                    );
                    if !has_value {
                        transitions.insert([transition.names[0].clone(), transition.names[1].clone()], default());
                    }
                    let existing_variants = transitions.get_mut(
                        &[transition.names[0].clone(), transition.names[1].clone()]
                    ).unwrap();
                    for (style, data) in vec_styles {
                        let potential_old_data = existing_variants.get_mut(&style);
                        let old_data;
                        if let Some(data) = potential_old_data {
                            old_data = data;
                        }
                        else {
                            existing_variants.insert(style, vec![]);
                            old_data = existing_variants.get_mut(&style).unwrap();
                        }
                        if let Some(animation) = variant.animation {
                            match &definitions[data[0]] {
                                ImageDefinition::Sprite(_image_handle) => {
                                    let mut file_handles = vec![];
                                    for i in 0..animation.number_of_states {
                                        file_handles.push(definitions[data[i * 2]].force_sprite());
                                    }
                                    old_data.push(TerrainRendering::AnimatedSprite(file_handles, animation));
                                }
                                ImageDefinition::SpriteSheet(_image_handle, (_image_width, _image_height)) => {
                                    let mut file_handles = vec![];
                                    for i in 0..animation.number_of_states {
                                        file_handles.push((definitions[data[i * 2]].force_sprite_sheet(), data[(i * 2) + 1]))
                                    }
                                    old_data.push(TerrainRendering::AnimatedSpriteSheet(file_handles, animation));
                                }
                            }
                        }
                        else {
                            match &definitions[data[0]] {
                                ImageDefinition::Sprite(image_handle) => {
                                    old_data.push(TerrainRendering::Sprite(image_handle.clone()));
                                }
                                ImageDefinition::SpriteSheet(image_handle, (image_width, image_height)) => {
                                    old_data.push(TerrainRendering::SpriteSheet(image_handle.clone(), *image_width, *image_height, data[1]));
                                }
                            }
                        }
                    }
                }
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
        static EXTENSIONS: &[&str] = &["nrf"];
        EXTENSIONS
    }
}

// This is gross. There must be a better way to do this (I know there is)
// But I don't know how I would do it and I don't care enough. It's *fine*.
fn conjoin_styles(styles: TerrainRenderingTransitionJSON) -> Vec<(TransitionType, Vec<usize>)> {
    let mut output = vec![];
    if let Some(value) = styles.central {
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
    return output;
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
struct TerrainDataJSON {
    minimum_height: usize,
    maximum_height: usize,
    states: Vec<TerrainStateJSON>,
}

struct TerrainData {
    minimum_height: usize,
    maximum_height: usize,
    states: Vec<TerrainState>,
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
struct TerrainStateJSON {
    name: String,
    approx_color: String,
    walk_sound: String,
    run_sound: String
}

struct TerrainState {
    name: String,
    approx_color: String,
    walk_sound: Handle<AudioSource>,
    run_sound: Handle<AudioSource>
}

#[derive(Deserialize)]
struct TerrainTransitionJSON {
    names: Vec<String>,
    meta_location: String,
}

struct TerrainTransition {
    names: Vec<String>,
    meta_data: Vec<(TransitionType, TerrainRendering)>
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
        return false;
    }
    fn collider_dimensions(&self) -> &[(f64, f64, f64, f64)] {
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
}

enum ImageDefinition {
    Sprite(Handle<Image>),
    SpriteSheet(Handle<Image>, (usize, usize))
}

impl ImageDefinition {
    fn force_sprite(&self) -> Handle<Image> {
        match self {
            Self::Sprite(handle) => return handle.clone(),
            Self::SpriteSheet(_, _) => panic!()
        }
    }
    fn force_sprite_sheet(&self) -> (Handle<Image>, usize, usize) {
        match self {
            Self::Sprite(_) => panic!(),
            Self::SpriteSheet(handle, (width, height)) =>
                return (handle.clone(), *width, *height)
        }
    }
}

#[derive(Clone)]
pub enum TerrainRendering {
    /// image
    Sprite(Handle<Image>),
    /// image, width, height, index
    SpriteSheet(Handle<Image>, usize, usize, usize),
    /// [image], animation
    AnimatedSprite(Vec<Handle<Image>>, AnimationInfo),
    /// [image, width, height, index], animation
    AnimatedSpriteSheet(Vec<((Handle<Image>, usize, usize), usize)>, AnimationInfo)
}

#[derive(Deserialize)]
struct TerrainRenderingJSON {
    files: Vec<TerrainRenderingFileJSON>,
    variants: Vec<TerrainRenderingTransitionJSON>,
}

#[derive(Deserialize)]
struct TerrainRenderingFileJSON {
    location: String,
    width: usize,
    height: usize
}

#[derive(Deserialize, Clone)]
struct TerrainRenderingTransitionJSON {
    animation: Option<AnimationInfo>,
    central: Option<Vec<usize>>,
    up: Option<Vec<usize>>,
    down: Option<Vec<usize>>,
    left: Option<Vec<usize>>,
    right: Option<Vec<usize>>,
    up_left: Option<Vec<usize>>,
    up_right: Option<Vec<usize>>,
    down_left: Option<Vec<usize>>,
    down_right: Option<Vec<usize>>,
    inverted_up_left: Option<Vec<usize>>,
    inverted_up_right: Option<Vec<usize>>,
    inverted_down_left: Option<Vec<usize>>,
    inverted_down_right: Option<Vec<usize>>
}

#[derive(Deserialize, Clone, Copy)]
pub struct AnimationInfo {
    pub number_of_states: usize,
    pub ticks_between_states: usize,
}
