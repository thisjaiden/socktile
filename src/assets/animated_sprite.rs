use std::time::Duration;

use crate::prelude::*;
use bevy::{utils::BoxedFuture, reflect::TypeUuid, asset::{AssetLoader, LoadContext, LoadedAsset, AssetPath}};

#[derive(TypeUuid, Deserialize, Component, Clone)]
#[uuid = "0789aad4-6f48-4721-a492-704cdf0f303a"]
pub struct AnimatedSprite {
    #[serde(skip)]
    #[serde(default)]
    images: Vec<Handle<Image>>,
    image_locations: Vec<String>,
    number_of_frames: usize,
    // in ms
    delay_between_frames: usize,
    end_behavior: EndBehavior,
    #[serde(default)]
    current_time: Duration,
    #[serde(default)]
    current_frame: usize,
    #[serde(default)]
    stalled: bool
}

impl AnimatedSprite {
    pub fn update(&mut self, time: Time, writeable: &mut Handle<Image>, blank: Handle<Image>) {
        info!("updating!");
        self.current_time += time.delta();
        if !self.stalled {
            let frame = self.current_time.as_millis() as usize / self.delay_between_frames;
            if frame != self.current_frame {
                if frame > self.number_of_frames {
                    match self.end_behavior {
                        EndBehavior::Stall => {
                            self.stalled = true;
                        },
                        EndBehavior::Blank => {
                            self.stalled = true;
                            writeable.set(Box::new(blank)).unwrap();
                        },
                        EndBehavior::Repeat => {
                            self.current_time = Duration::ZERO;
                            self.current_frame = 0;
                            writeable.set(Box::new(self.images[0].clone())).unwrap();
                        }
                    }
                }
                else {
                    self.current_frame = frame;
                    writeable.set(Box::new(self.images[frame].clone())).unwrap();
                }
            }
        }
    }
}

#[derive(Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum EndBehavior {
    Stall,
    Blank,
    Repeat
}

pub struct AnimatedSpriteLoader;

impl AssetLoader for AnimatedSpriteLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let mut partially_propigated: AnimatedSprite = serde_json::from_slice(bytes)?;
            let mut dependencies = vec![];
            for sample in &partially_propigated.image_locations {
                let path: AssetPath = load_context
                    .path()
                    .parent()
                    .unwrap()
                    .join(format!("{}", sample))
                    .into();
                partially_propigated.images.push(load_context.get_handle(path.clone()));
                dependencies.push(path);
            }

            partially_propigated.stalled = false;
            
            let loaded_asset = LoadedAsset::new(partially_propigated);
            load_context.set_default_asset(loaded_asset.with_dependencies(dependencies));
            Ok(())
        })
    }
    fn extensions(&self) -> &[&str] {
        &["ajson"]
    }
}
