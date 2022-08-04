use crate::prelude::*;
use bevy::{utils::{HashMap, BoxedFuture}, reflect::TypeUuid, asset::{AssetLoader, LoadContext, LoadedAsset, AssetPath}};
use bevy_kira_audio::AudioSource;

#[derive(TypeUuid)]
#[uuid = "184160fa-44b9-4ddb-a72d-3d945adc306d"]
pub struct AudioSamples {
    keys: HashMap<String, Handle<AudioSource>>
}

impl AudioSamples {
    pub fn get(&self, name: &str) -> Handle<AudioSource> {
        if let Some(handle) = self.keys.get(name) {
            return handle.clone();
        }
        error!("Unable to find an audio sample with the name '{}'", name);
        panic!("{FATAL_ERROR}");
    }
    pub fn _exists(&self, key: &str) -> bool {
        self.keys.contains_key(&key.to_string())
    }
    pub fn _get_checked(&self, key: &str) -> Option<Handle<AudioSource>> {
        let potential_value = self.keys.get(&key.to_string());
        if let Some(value) = potential_value {
            Some(value.clone())
        }
        else {
            warn!("No value found for audio key {}", key);
            None
        }
    }
}

pub struct AudioSamplesLoader;

impl AssetLoader for AudioSamplesLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let mut keys: HashMap<String, Handle<AudioSource>> = default();
            let meta: Vec<AudioSample> = serde_json::from_slice(bytes)?;
            let mut dependencies = vec![];
            // audio dependencies
            for sample in meta {
                let path: AssetPath = load_context
                    .path()
                    .parent()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join(format!("audio/{}", sample.meta_location))
                    .into();
                keys.insert(sample.name, load_context.get_handle(path.clone()));
                dependencies.push(path);
            }
            info!("{} audio samples loaded", keys.len());
            
            let loaded_asset = LoadedAsset::new(AudioSamples { keys });
            load_context.set_default_asset(loaded_asset.with_dependencies(dependencies));
            Ok(())
        })
    }
    fn extensions(&self) -> &[&str] {
        &["sjson"]
    }
}

#[derive(Deserialize, Debug)]
struct AudioSample {
    pub name: String,
    pub meta_location: String
}
