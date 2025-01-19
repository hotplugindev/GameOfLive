use bevy::prelude::*;

/// Resource defining the player's movement speed
#[derive(Resource)]
pub struct GameSpeed(pub f32);
