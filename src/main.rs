use bevy::prelude::*;

mod components;
mod systems;
mod resources;

use components::*;
use systems::*;
use resources::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(PlayerSpeed(300.0))
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, move_player)
        .run();
}
