use bevy::prelude::*;
use rand::Rng;
use crate::{components::Node, resources::GameSpeed, components::Camera};

const GRID_SIZE: usize = 20;
const NODE_SIZE: f32 = 20.0;
const WINDOW_SIZE: f32 = (GRID_SIZE as f32) * NODE_SIZE;

pub fn setup(mut commands: Commands){
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(100.0, 200.0, 0.0),
            ..default()
        },
        Camera,
    ));
}

pub fn spawn_objects(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {

    let mut rng = rand::thread_rng();

    // Spawn nodes randomly
    for _ in 0..50 {
        let position = IVec2::new(rng.gen_range(0..GRID_SIZE as i32), rng.gen_range(0..GRID_SIZE as i32));
        commands
            .spawn_batch(SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::splat(NODE_SIZE)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        position.x as f32 * NODE_SIZE - WINDOW_SIZE / 2.0 + NODE_SIZE / 2.0,
                        position.y as f32 * NODE_SIZE - WINDOW_SIZE / 2.0 + NODE_SIZE / 2.0,
                        0.0,
                    ),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Node { position });
    }
}


pub fn update_positions(mut query: Query<(&mut Node, &mut Transform)>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let mut nodes = Vec::new();

    // Collect all node positions
    for (node, _) in query.iter() {
        nodes.push(node.position);
    }

    let mut rng = rand::thread_rng();

    // Update each node
    for (mut node, mut transform) in query.iter_mut() {
        // Randomly pick a direction (including staying in place)
        let direction = IVec2::new(rng.gen_range(-1..=1), rng.gen_range(-1..=1));
        let new_position = node.position + direction;

        // Check bounds
        if new_position.x >= 0
            && new_position.x < GRID_SIZE as i32
            && new_position.y >= 0
            && new_position.y < GRID_SIZE as i32
        {
            node.position = new_position;
        }

        // Update transform to match grid position
        transform.translation = Vec3::new(
            node.position.x as f32 * NODE_SIZE - WINDOW_SIZE / 2.0 + NODE_SIZE / 2.0,
            node.position.y as f32 * NODE_SIZE - WINDOW_SIZE / 2.0 + NODE_SIZE / 2.0,
            0.0,
        );

        // Log relative positions for demonstration (optional)
        for other_position in &nodes {
            if *other_position != node.position {
                let relative = *other_position - node.position;
                println!("Node at {:?} sees relative position {:?}", node.position, relative);
            }
        }
    }
}

