use crate::prelude::*;
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    reflect::{TypeUuid, TypePath},
    utils::{BoxedFuture, HashMap},
};
use serde_json::Value;

#[derive(TypeUuid, TypePath)]
#[uuid = "fbf9ee66-3a2b-433d-9329-fc2681eb3a47"]
pub struct LanguageKeys {
    keys: HashMap<String, LanguageValue>,
}

impl LanguageKeys {
    pub fn get(&self, key: &str) -> String {
        let potential_value = self.keys.get(&key.to_string());
        if let Some(value) = potential_value {
            match value {
                LanguageValue::Value(val) => val.clone(),
                LanguageValue::RandomValue(vals) => rand_from_array(vals.to_vec()),
            }
        }
        else {
            warn!("No value found for language key {}", key);
            key.to_string()
        }
    }
    pub fn _exists(&self, key: &str) -> bool {
        self.keys.contains_key(&key.to_string())
    }
    pub fn _get_checked(&self, key: &str) -> Option<String> {
        let potential_value = self.keys.get(&key.to_string());
        if let Some(value) = potential_value {
            match value {
                LanguageValue::Value(val) => Some(val.clone()),
                LanguageValue::RandomValue(vals) => Some(rand_from_array(vals.to_vec())),
            }
        }
        else {
            warn!("No value found for language key {}", key);
            None
        }
    }
}

#[derive(Debug)]
enum LanguageValue {
    Value(String),
    RandomValue(Vec<String>),
}

pub struct LanguageKeysLoader;

impl AssetLoader for LanguageKeysLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let mut keys: HashMap<String, LanguageValue> = default();
            let lang_core = serde_json::from_slice(bytes)?;
            let fresh_keys = grab_keys_recursively(
                load_context.path().file_stem().unwrap().to_str().unwrap(),
                lang_core,
            );
            for (key, value) in fresh_keys {
                keys.insert(key, value);
            }
            info!("{} language keys loaded", keys.len());
            let loaded_asset = LoadedAsset::new(LanguageKeys { keys });
            load_context.set_default_asset(loaded_asset);
            Ok(())
        })
    }
    fn extensions(&self) -> &[&str] {
        &["ljson"]
    }
}

/// Takes the keys out of a json object and monosizes them into (Key, Value) pairs.
/// Subobjects are appended with a .[object] phrase
fn grab_keys_recursively(current_key: &str, current_value: Value) -> Vec<(String, LanguageValue)> {
    let mut returnable = vec![];
    for (key, value) in current_value.as_object().unwrap() {
        if value.is_string() {
            returnable.push((
                format!("{}.{}", current_key, key),
                LanguageValue::Value(value.as_str().unwrap().to_string()),
            ));
        }
        if value.is_array() {
            let mut smallarray = vec![];
            for element in value.as_array().unwrap() {
                smallarray.push(element.as_str().unwrap().to_string());
            }
            returnable.push((
                format!("{}.{}", current_key, key),
                LanguageValue::RandomValue(smallarray),
            ));
        }
        if value.is_object() {
            returnable.append(&mut grab_keys_recursively(
                &format!("{}.{}", current_key, key),
                value.clone(),
            ));
        }
    }
    returnable
}
