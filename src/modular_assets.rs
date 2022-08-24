use bevy::{
    utils::HashMap,
    reflect::TypeUuid,
    asset::{
        AssetLoader,
        LoadContext,
        BoxedFuture,
        LoadedAsset,
        AssetPath
    }
};

use crate::prelude::{*, language::LanguageKeysLoader, audio::AudioSamplesLoader, tiles::{TileTypeConfig, TileTypeConfigLoader, TileTransitionMasterConfig, TileTransitionMasterConfigLoader, TileTransitionConfig, TileTransitionConfigLoader, Variant}};

#[derive(Default)]
pub struct ModularAssetsPlugin;

impl Plugin for ModularAssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_asset::<ModularAssets>()
            //.add_asset_loader(ModularAssetsLoader)
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

/*
#[derive(TypeUuid, Debug)]
#[uuid = "8d513cb4-0fa2-4069-b6ad-fb7e8dd37031"]
pub struct ModularAssets {
    terrain_data: TerrainData
}

impl ModularAssets {
    // NOTE: INPUT ENVIRONMENT IS FLIPPED VERTICALLY (IN HUMAN LOGICAL ORDER)
    pub fn get_tile(&self, environment: [usize; 9], harsh: bool) -> (TerrainRendering, TransitionType) {
        let maybe_transition = self.get_transition_type(environment);
        if let Some((mut transition, main, sub)) = maybe_transition {
            let rendering = self.get_terrain_rendering(main, sub, &mut transition, harsh);
            (rendering, transition)
        }
        else {
            // no valid transition is known. fallback time!
            let rendering = self.get_terrain_rendering(environment[4], environment[4], &mut TransitionType::Center, false);
            (rendering, TransitionType::Center)
        }
    }
    /// Finds the appropriate rendering for a given terrain type and transition type
    fn get_terrain_rendering(&self, terrain_id: usize, alt_id: usize, transition: &mut TransitionType, harsh: bool) -> TerrainRendering {
        let central = self.terrain_data.states[terrain_id].name.clone();
        let non_central = self.terrain_data.states[alt_id].name.clone();
        let transitions_maybe = self.terrain_data.transitions.get(&(central.clone(), non_central.clone(), harsh));
        if let Some(transitions_map) = transitions_maybe {
            let types_maybe = transitions_map.get(transition);
            if let Some(types) = types_maybe {
                rand_from_array(types.to_vec())
            }
            else {
                warn!("No transition {:?} for materials {} and {}, falling back", transition, central, non_central);
                trace!("{:#?}", self);
                todo!();
            }
        }
        else {
            // this is a submissive terrain state, so we just use the central point
            if self.terrain_data.transitions.get(&(non_central.clone(), central.clone(), harsh)).is_some() {
                *transition = TransitionType::Center;
                self.get_terrain_rendering(terrain_id, terrain_id, transition, false)
            }
            else {
                error!("No transition between materials {} and {} (harsh = {harsh})", central, non_central);
                panic!("{FATAL_ERROR}");
            }
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
                terrain_data: default()
            };
            let terrain_core: TerrainDataJSON;
            let transition_core: Vec<TerrainTransitionJSON>;

            if EMBED_ASSETS {
                terrain_core = serde_json::from_slice(include_bytes!("../assets/metadata/terrain.json")).unwrap();
                transition_core = serde_json::from_slice(include_bytes!("../assets/metadata/transitions.json")).unwrap();
            }
            else {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    terrain_core = serde_json::from_str(&std::fs::read_to_string("../assets/metadata/terrain.json").unwrap()).unwrap();
                    transition_core = serde_json::from_str(&std::fs::read_to_string("../assets/metadata/transitions.json").unwrap()).unwrap();
                }
                #[cfg(target_arch = "wasm32")]
                {
                    // TODO: This whole thing needs to be nuked and redone for better native wasm
                    terrain_core = TerrainDataJSON { minimum_height: 0, maximum_height: 5, states: vec![] };
                    transition_core = vec![]; //TerrainTransitionJSON { names: vec![], meta_location: String::new() };
                }
            }

            let mut dependencies = vec![];

            // terrain metadata
            final_out.terrain_data.maximum_height = terrain_core.maximum_height;
            final_out.terrain_data.minimum_height = terrain_core.minimum_height;
            // terrain definitions
            for definition in terrain_core.states {
                final_out.terrain_data.states.push(TerrainState {
                    name: definition.name,
                    approx_color: definition.approx_color,
                    walk_sound: definition.walk_sound,
                    run_sound: definition.run_sound
                });
            }
            info!("{} terrain states loaded", final_out.terrain_data.states.len());
            if DEV_BUILD {
                info!("Creating injectable.json based on states");
                let mut contents = String::from("\"intGridValues\":[");
                for (index, state) in final_out.terrain_data.states.iter().enumerate() {
                    if index > 0 {
                        contents += ",";
                    }
                    contents += "{\"value\":";
                    contents += &format!("{}", index + 1);
                    contents += ",\"identifier\":\"";
                    contents += &state.name;
                    contents += "\",\"color\":\"";
                    contents += &state.approx_color;
                    contents += "\"}";
                }
                contents += "],";
                #[cfg(not(target_arch = "wasm32"))]
                std::fs::write("./injectable.json", contents).unwrap();
                info!("injectable.json written");
            }
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
                
                let transitions: &mut HashMap<(String, String, bool), HashMap<TransitionType, Vec<TerrainRendering>>> = &mut final_out.terrain_data.transitions;
                // for every transition declaration
                for variant in meta.variants {
                    let vec_styles = conjoin_styles(variant.clone());
                    let has_value = transitions.contains_key(
                        &(transition.names[0].clone(), transition.names[1].clone(), variant.harsh)
                    );
                    if !has_value {
                        transitions.insert((transition.names[0].clone(), transition.names[1].clone(), variant.harsh), default());
                    }
                    let existing_variants = transitions.get_mut(
                        &(transition.names[0].clone(), transition.names[1].clone(), variant.harsh)
                    ).unwrap();
                    for (style, harsh, data) in vec_styles {
                        let potential_old_data = existing_variants.get_mut(&style);
                        let old_data = if let Some(data) = potential_old_data {
                            data
                        }
                        else {
                            existing_variants.insert(style, vec![]);
                            existing_variants.get_mut(&style).unwrap()
                        };
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
            info!("{} terrain transitions loaded", final_out.terrain_data.transitions.len());
            
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

#[derive(Deserialize)]
struct TerrainDataJSON {
    minimum_height: usize,
    maximum_height: usize,
    states: Vec<TerrainState>,
}

#[derive(Debug)]
struct TerrainData {
    minimum_height: usize,
    maximum_height: usize,
    states: Vec<TerrainState>,
    transitions: HashMap<(String, String, bool), HashMap<TransitionType, Vec<TerrainRendering>>>
}

impl Default for TerrainData {
    fn default() -> TerrainData {
        TerrainData {
            minimum_height: 0,
            maximum_height: 0,
            states: vec![],
            transitions: default()
        }
    }
}


#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
/// Represents the state of a single tile of terrain.
struct TerrainState {
    name: String,
    approx_color: String,
    walk_sound: String,
    run_sound: String
}

#[derive(Deserialize)]
struct TerrainTransitionJSON {
    names: Vec<String>,
    meta_location: String,
}

enum ImageDefinition {
    Sprite(Handle<Image>),
    SpriteSheet(Handle<Image>, (usize, usize))
}

impl ImageDefinition {
    fn force_sprite(&self) -> Handle<Image> {
        match self {
            Self::Sprite(handle) => handle.clone(),
            Self::SpriteSheet(_, _) => panic!()
        }
    }
    fn force_sprite_sheet(&self) -> (Handle<Image>, usize, usize) {
        match self {
            Self::Sprite(_) => panic!(),
            Self::SpriteSheet(handle, (width, height)) =>
                (handle.clone(), *width, *height)
        }
    }
}

#[derive(Clone, Debug)]
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
    harsh: bool,
    animation: Option<AnimationInfo>,
    center: Option<Vec<usize>>,
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

#[derive(Deserialize, Clone, Copy, Debug)]
pub struct AnimationInfo {
    pub number_of_states: usize,
    pub ticks_between_states: usize,
}

*/

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


