use crate::prelude::*;
use crate::animated_sprite::AnimatedSpriteLoader;

use crate::audio::AudioSamplesLoader;
use crate::language::LanguageLoader;
use crate::language::{SingleLanguage, SingleLanguageLoader};

#[derive(Default)]
pub struct ModularAssetsPlugin;

impl Plugin for ModularAssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_asset::<Language>()
            .add_asset_loader(LanguageLoader)
            .add_asset::<SingleLanguage>()
            .add_asset_loader(SingleLanguageLoader)
            .add_asset::<AudioSamples>()
            .add_asset_loader(AudioSamplesLoader)
            .add_asset::<AnimatedSprite>()
            .add_asset_loader(AnimatedSpriteLoader);
    }
}
