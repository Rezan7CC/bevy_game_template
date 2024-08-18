use crate::asset_loading::TextureAssets;
use crate::game_input;
use bevy::prelude::*;
use framework::gameplay::{movement, player};
use framework::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), system_spawn_player);

        app.add_systems(
            Update,
            (system_player_movement_input,).run_if(in_state(GameState::Game)),
        );
    }
}

fn system_spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: textures.bevy_icon.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        },
        player::Player::default(),
        movement::Movement::default(),
        movement::MovementParameters {
            rotate_to_control_input: false,
            max_speed: 300.0,
            acceleration: 2000.0,
            deceleration: 1000.0,
            use_physics_system: false,
            direction_behavior: movement::MovementDirectionBehavior::InputAffectsVelocityDirection,
            default_forward_direction: Vec3::Y,
            ..Default::default()
        },
    ));
}

pub fn system_player_movement_input(
    input_actions: Res<game_input::GameInputActionsResource>,
    mut query: Query<&mut movement::MovementParameters, With<player::Player>>,
) {
    for mut movement_params in query.iter_mut() {
        movement_params.control_input = Vec3::ZERO;

        if input_actions.action_active(game_input::GameInputActionType::MoveUp) {
            movement_params.control_input += Vec3::Y;
        }

        if input_actions.action_active(game_input::GameInputActionType::MoveDown) {
            movement_params.control_input += -Vec3::Y;
        }

        if input_actions.action_active(game_input::GameInputActionType::MoveLeft) {
            movement_params.control_input += -Vec3::X;
        }

        if input_actions.action_active(game_input::GameInputActionType::MoveRight) {
            movement_params.control_input += Vec3::X;
        }

        movement_params.control_input = movement_params.control_input.normalize_or_zero();
    }
}
