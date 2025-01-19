use bevy::prelude::*;

/// Marker component for the player entity
#[derive(Component)]
pub struct Node{
    pub position: IVec2,
}

#[derive(Component)]
pub struct Camera;
