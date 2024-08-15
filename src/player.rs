use crate::actions::Actions;
use crate::asset_loading::TextureAssets;
use bevy::prelude::*;
use framework::gameplay::{movement, player};
use framework::GameState;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), system_spawn_player);

        app.add_systems(Update, system_move_player.run_if(in_state(GameState::Game)));
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
            max_speed: 250.0,
            acceleration: 1500.0,
            deceleration: 1000.0,
            use_physics_system: false,
            direction_behavior:
                movement::MovementDirectionBehavior::InputOverridesVelocityDirection,
            default_forward_direction: Vec3::Y,
            ..Default::default()
        },
    ));
}

fn system_move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<player::Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut player_transform in &mut player_query {
        player_transform.translation += movement;
    }
}
