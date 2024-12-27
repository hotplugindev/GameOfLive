use bevy::prelude::*;

/// Resource defining the player's movement speed
#[derive(Resource)]
pub struct PlayerSpeed(pub f32);
