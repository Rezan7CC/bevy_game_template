use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use framework::GameState;

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::StartupLoading)
                .load_collection::<AudioAssets>()
                .load_collection::<TextureAssets>(),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/EmbershadeIcon_HighContrast_TransparentBG.png")]
    pub studio_icon: Handle<Image>,

    #[asset(path = "textures/discord-mark-white.png")]
    pub discord_icon: Handle<Image>,
}
