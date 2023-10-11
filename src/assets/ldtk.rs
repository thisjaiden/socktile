use crate::prelude::*;
use bevy::{reflect::{TypeUuid, TypePath}, asset::AssetLoader};
use bevy_kira_audio::AudioSource;
use crate::ldtk_quicktype::*;

#[derive(TypeUuid, TypePath)]
#[uuid = "a0b47c49-5d1c-431f-b272-68eff3e58448"]
pub struct EngineProject {
    raw_project: LdtkJson,
}

pub struct EngineProjectLoader;

impl AssetLoader for EngineProjectLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
        let model: LdtkJson = serde_json::from_slice(bytes).unwrap();
        todo!()
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}

enum EngineAsset {
    Image(Handle<Image>),
    Font(Handle<Font>),
    Audio(Handle<AudioSource>)
}
