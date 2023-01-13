use crate::prelude::*;
use bevy::{
    asset::{AssetLoader, AssetPath, LoadContext, LoadedAsset},
    reflect::TypeUuid,
    utils::{BoxedFuture, HashMap},
};

#[derive(Deserialize, TypeUuid)]
#[uuid = "184160fa-44b9-4ddb-a72d-3d945adc306e"]
pub struct TileTypeConfig {
    pub states: Vec<TerrainState>,
}

pub struct TileTypeConfigLoader;

impl AssetLoader for TileTypeConfigLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let tile_type_config: TileTypeConfig = serde_json::from_slice(bytes)?;
            info!("{} terrain states loaded", tile_type_config.states.len());
            if DEV_BUILD {
                #[cfg(not(any(target_arch = "wasm32", target_os = "ios")))]
                {
                    info!("Creating injectable.json based on states");
                    let mut contents = String::from("\"intGridValues\":[");
                    for (index, state) in tile_type_config.states.iter().enumerate() {
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
                    std::fs::write("./injectable.json", contents).unwrap();
                    info!("injectable.json written");
                }
            }
            let loaded_asset = LoadedAsset::new(tile_type_config);
            load_context.set_default_asset(loaded_asset);
            Ok(())
        })
    }
    fn extensions(&self) -> &[&str] {
        &["tjson"]
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
/// Represents the state of a single tile of terrain.
pub struct TerrainState {
    pub name: String,
    approx_color: String,
    walk_sound: String,
    run_sound: String,
}

#[derive(TypeUuid)]
#[uuid = "184160fa-44b9-4ddb-a72d-3d945adc306f"]
pub struct TileTransitionMasterConfig {
    pub transitions: HashMap<[String; 2], Handle<TileTransitionConfig>>,
}

pub struct TileTransitionMasterConfigLoader;

impl AssetLoader for TileTransitionMasterConfigLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let mut out_trans: HashMap<[String; 2], Handle<TileTransitionConfig>> = default();
            let transitions: Vec<TerrainTransition> = serde_json::from_slice(bytes)?;
            let mut dependencies = vec![];
            for transition in transitions {
                let path: AssetPath = load_context
                    .path()
                    .parent()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join(format!("terrain/{}", transition.meta_location))
                    .into();
                out_trans.insert(
                    [transition.names[0].clone(), transition.names[1].clone()],
                    load_context.get_handle(path.clone()),
                );
                dependencies.push(path);
            }
            let m_conf = TileTransitionMasterConfig {
                transitions: out_trans,
            };
            let loaded_asset = LoadedAsset::new(m_conf).with_dependencies(dependencies);
            load_context.set_default_asset(loaded_asset);
            Ok(())
        })
    }
    fn extensions(&self) -> &[&str] {
        &["ujson"]
    }
}

#[derive(Deserialize)]
struct TerrainTransition {
    names: Vec<String>,
    meta_location: String,
}

#[derive(TypeUuid, Clone)]
#[uuid = "184160fa-44b9-4ddb-a72d-3d945adc3070"]
pub struct TileTransitionConfig {
    pub images: Vec<ImageDefinition>,
    pub variants: Vec<Variant>,
}

pub struct TileTransitionConfigLoader;

impl AssetLoader for TileTransitionConfigLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
        Box::pin(async move {
            let core: TerrainRenderingJSON = serde_json::from_slice(bytes)?;
            let mut dependencies = vec![];
            let mut final_out = TileTransitionConfig {
                images: vec![],
                variants: vec![],
            };
            for file in core.files {
                let path: AssetPath = load_context
                    .path()
                    .parent()
                    .unwrap()
                    .join(file.location)
                    .into();
                if file.width == 1 && file.height == 1 {
                    final_out.images.push(ImageDefinition::Sprite(
                        load_context.get_handle(path.clone()),
                    ));
                }
                else {
                    final_out.images.push(ImageDefinition::SpriteSheet(
                        load_context.get_handle(path.clone()),
                        (file.width, file.height),
                    ));
                }
                dependencies.push(path);
            }
            for variant in core.variants {
                final_out.variants.push(variant);
            }
            let loaded_asset = LoadedAsset::new(final_out).with_dependencies(dependencies);
            load_context.set_default_asset(loaded_asset);
            Ok(())
        })
    }
    fn extensions(&self) -> &[&str] {
        &["vjson"]
    }
}

#[derive(Deserialize)]
struct TerrainRenderingJSON {
    files: Vec<TerrainRenderingFileJSON>,
    variants: Vec<Variant>,
}

#[derive(Deserialize)]
struct TerrainRenderingFileJSON {
    location: String,
    width: usize,
    height: usize,
}

#[derive(Deserialize, Clone)]
pub struct Variant {
    // tp - top
    // lf - left
    // bt - bottom
    // ri - right
    // tl - top left
    // tr - top right
    // bl - bottom left
    // br - bottom right
    // nz - nothing
    pub animation: Option<AnimationInfo>,
    pub tl: Option<Vec<usize>>,
    pub tr: Option<Vec<usize>>,
    pub tp: Option<Vec<usize>>,
    pub ri: Option<Vec<usize>>,
    pub bt: Option<Vec<usize>>,
    pub bl: Option<Vec<usize>>,
    pub br: Option<Vec<usize>>,
    pub lf: Option<Vec<usize>>,
    pub nz: Option<Vec<usize>>,
    pub tlbl: Option<Vec<usize>>,
    pub tlbr: Option<Vec<usize>>,
    pub trbl: Option<Vec<usize>>,
    pub tltr: Option<Vec<usize>>,
    pub tpbl: Option<Vec<usize>>,
    pub tpbr: Option<Vec<usize>>,
    pub lfbr: Option<Vec<usize>>,
    pub ribl: Option<Vec<usize>>,
    pub tplf: Option<Vec<usize>>,
    pub tpri: Option<Vec<usize>>,
    pub lfbt: Option<Vec<usize>>,
    pub tpbt: Option<Vec<usize>>,
    pub lfri: Option<Vec<usize>>,
    pub btri: Option<Vec<usize>>,
    pub bttl: Option<Vec<usize>>,
    pub bttr: Option<Vec<usize>>,
    pub lftr: Option<Vec<usize>>,
    pub ritl: Option<Vec<usize>>,
    pub blbr: Option<Vec<usize>>,
    pub trbr: Option<Vec<usize>>,
    pub tpblbr: Option<Vec<usize>>,
    pub bttltr: Option<Vec<usize>>,
    pub tplfbr: Option<Vec<usize>>,
    pub tplfri: Option<Vec<usize>>,
    pub tplfbt: Option<Vec<usize>>,
    pub tpribl: Option<Vec<usize>>,
    pub lfbttr: Option<Vec<usize>>,
    pub lfbtri: Option<Vec<usize>>,
    pub tpbtri: Option<Vec<usize>>,
    pub btritl: Option<Vec<usize>>,
    pub tlblbr: Option<Vec<usize>>,
    pub trblbr: Option<Vec<usize>>,
    pub lftrbr: Option<Vec<usize>>,
    pub ritlbl: Option<Vec<usize>>,
    pub tltrbl: Option<Vec<usize>>,
    pub tltrbr: Option<Vec<usize>>,
    pub tplfbtri: Option<Vec<usize>>,
    pub tltrblbr: Option<Vec<usize>>,
}

#[derive(Deserialize, Clone, Copy, Debug)]
pub struct AnimationInfo {
    pub number_of_states: usize,
    pub ticks_between_states: usize,
}

#[derive(Clone)]
pub enum ImageDefinition {
    Sprite(Handle<Image>),
    SpriteSheet(Handle<Image>, (usize, usize)),
}

impl ImageDefinition {
    pub fn force_sprite(&self) -> Handle<Image> {
        match self {
            Self::Sprite(handle) => handle.clone(),
            Self::SpriteSheet(_, _) => panic!(),
        }
    }
    pub fn force_sprite_sheet(&self) -> (Handle<Image>, usize, usize) {
        match self {
            Self::Sprite(_) => panic!(),
            Self::SpriteSheet(handle, (width, height)) =>
                (handle.clone(), *width, *height)
        }
    }
}
