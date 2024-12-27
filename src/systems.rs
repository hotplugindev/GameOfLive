use bevy::prelude::*;
use crate::{components::Player, resources::PlayerSpeed, components::MyCameraMarker};

pub fn setup(mut commands: Commands){
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(100.0, 200.0, 0.0),
            ..default()
        },
        MyCameraMarker,
    ));
}

/// System to spawn the player
pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        Player,
    ));
    println!("Spawned a player!");
}

/// System to move the player using WASD keys
pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    player_speed: Res<PlayerSpeed>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        // WASD key handling
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
            println!("Moved Up");
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
            println!("Moved down");
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
            println!("Moved left");
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
            println!("Moved right");
        }

        transform.translation += direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();
    }
}
