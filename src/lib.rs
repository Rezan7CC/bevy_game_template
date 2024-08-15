#![allow(clippy::type_complexity)]

mod actions;
mod asset_loading;
mod menu;
mod player;

use crate::actions::ActionsPlugin;
use crate::asset_loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

use bevy::app::App;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(framework::FrameworkPluginGroup::default())
            .add_plugins((LoadingPlugin, MenuPlugin, ActionsPlugin, PlayerPlugin));
    }
}
