#![allow(clippy::type_complexity)]

mod asset_loading;
mod game_input;
mod menu;
mod player;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(framework::FrameworkPluginGroup::default())
            .add_plugins((
                asset_loading::AssetLoadingPlugin,
                menu::MenuPlugin,
                player::PlayerPlugin,
                game_input::GameInputPlugin::default(),
            ));
    }
}
