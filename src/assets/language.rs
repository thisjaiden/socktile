use crate::prelude::*;
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset, AssetPath},
    reflect::{TypeUuid, TypePath},
    utils::{BoxedFuture, HashMap},
};
use serde_json::Value;

pub fn system_post_load(
    core: Res<CoreAssets>,
    mut lang_serve: ResMut<Assets<Language>>,
    singles: Res<Assets<SingleLanguage>>
) {
    let lang = lang_serve.get_mut(&core.lang).unwrap();
    lang.load_waiting_handles(singles);
}

#[derive(TypeUuid, TypePath)]
#[uuid = "fbf9ee66-3a2b-433d-9329-fc2681eb3a47"]
pub struct Language {
    default_language: String,
    current_language: String,
    languages: HashMap<String, HashMap<String, LanguageValue>>,
    waiting_handles: Vec<(String, Handle<SingleLanguage>)>,
}

impl Language {
    fn load_waiting_handles(&mut self, controller: Res<Assets<SingleLanguage>>) {
        for (name, handle) in &self.waiting_handles {
            self.languages.insert(
                name.to_string(),
                controller.get(handle).unwrap().get_all_data()
            );
        }
        self.waiting_handles.clear();
    }
    pub fn set_language(&mut self, language: String) {
        self.current_language = language.clone();
    }
    pub fn get_key(&self, key: &str) -> String {
        self.get_key_by_language(&self.current_language, key)
    }
    pub fn get_key_by_language(&self, language: &str, key: &str) -> String {
        if let Some(lang_data) = self.languages.get(language) {
            if let Some(result) = lang_data.get(key) {
                match result {
                    LanguageValue::Value(val) =>
                        val.clone(),
                    LanguageValue::RandomValue(vals) =>
                        rand_from_array(vals.to_vec()),
                }
            }
            else {
                todo!();
            }
        }
        else if self.default_language != language {
            warn!("Unknown language ({language}), using default.");
            if let Some(lang_data) = self.languages.get(language) {
                if let Some(result) = lang_data.get(key) {
                    match result {
                        LanguageValue::Value(val) =>
                            return val.clone(),
                        LanguageValue::RandomValue(vals) =>
                            return rand_from_array(vals.to_vec()),
                    }
                }
                else {
                    warn!("Unable to find key {key} for the default language.");
                    return String::new();
                }
            }
            else {
                error!("Default language not found.");
                panic!("{}", FATAL_ERROR);
            }
        }
        else {
            error!("Default language not found.");
            panic!("{}", FATAL_ERROR);
        }
    }
}

#[derive(Clone, Debug)]
enum LanguageValue {
    Value(String),
    RandomValue(Vec<String>),
}

pub struct LanguageLoader;

impl AssetLoader for LanguageLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let mut output = Language {
                default_language: String::from("American English"),
                current_language: String::from("American English"),
                languages: default(),
                waiting_handles: vec![]
            };
            let core: Value = serde_json::from_slice(bytes)?;
            let mut dependencies = vec![];
            for language in core["languages"].as_array().unwrap() {
                let lang_name = language["name"].as_str().unwrap();
                let lang_loc = language["file"].as_str().unwrap();
                let dep_path: AssetPath = load_context
                    .path()
                    .parent().unwrap()
                    .parent().unwrap()
                    .join("lang")
                    .join(lang_loc)
                    .into();
                dependencies.push(dep_path.clone());
                let handle = load_context.get_handle(dep_path);
                output.waiting_handles.push((String::from(lang_name), handle));
            }
            
            let loaded_asset = LoadedAsset::new(output);
            load_context.set_default_asset(loaded_asset.with_dependencies(dependencies));
            info!("all language data loaded");
            Ok(())
        })
    }
    fn extensions(&self) -> &[&str] {
        &["bjson"]
    }
}

#[derive(TypeUuid, TypePath)]
#[uuid = "f472b83c-fc31-49ea-8b00-0bf8a3bc36cd"]
pub struct SingleLanguage {
    data: HashMap<String, LanguageValue>
}

impl SingleLanguage {
    fn get_all_data(&self) -> HashMap<String, LanguageValue> {
        self.data.clone()
    }
}

pub struct SingleLanguageLoader;

impl AssetLoader for SingleLanguageLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let mut keys: HashMap<String, LanguageValue> = default();
            let language_data = serde_json::from_slice(bytes)?;
            let fresh_keys = grab_keys_recursively("", language_data);
            for (key, value) in fresh_keys {
                keys.insert(key, value);
            }
            let loaded_asset: LoadedAsset<_> = LoadedAsset::new(SingleLanguage {
                data: keys
            });
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
