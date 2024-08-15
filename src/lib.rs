#![allow(clippy::type_complexity)]

mod actions;
mod asset_loading;
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
                actions::ActionsPlugin,
                player::PlayerPlugin,
            ));
    }
}
