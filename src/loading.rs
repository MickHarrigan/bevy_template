use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu),
        )
        .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, Skyboxes>(GameState::Loading);
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,
    #[asset(path = "textures/github.png")]
    pub github: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct Skyboxes {
    #[asset(path = "textures/skyboxes/ForbiddenCity/cubemap.png")]
    pub city: Handle<Image>,
    #[asset(path = "textures/skyboxes/SaintPetersBasilica/cubemap.png")]
    pub church: Handle<Image>,
    #[asset(path = "textures/skyboxes/Tallinn/cubemap.png")]
    pub town_square: Handle<Image>,
    #[asset(path = "textures/skyboxes/Brudslojan/cubemap.png")]
    pub mountainside: Handle<Image>,
    #[asset(path = "textures/skyboxes/MountainPath/cubemap.png")]
    pub forest: Handle<Image>,
}
