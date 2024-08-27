use bevy::prelude::*;
use strum_macros::EnumIter;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, EnumIter, PartialEq, Copy, Clone)]
pub enum GameInputActionType {
    MoveUp,
    MoveDown,
    MoveRight,
    MoveLeft,
}

impl framework::input::InputActionType for GameInputActionType {
    fn to_keycodes(&self) -> Vec<KeyCode> {
        match self {
            GameInputActionType::MoveUp => vec![KeyCode::KeyW],
            GameInputActionType::MoveDown => vec![KeyCode::KeyS],
            GameInputActionType::MoveRight => vec![KeyCode::KeyD],
            GameInputActionType::MoveLeft => vec![KeyCode::KeyA],
        }
    }
}

pub type GameInputActionsResource = framework::input::InputActionsResource<GameInputActionType>;
pub type GameInputPlugin = framework::input::InputPlugin<GameInputActionType>;
