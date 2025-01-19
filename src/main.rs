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
        .insert_resource(GameSpeed(300.0))
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_objects)
        .add_systems(Update, update_positions)
        .run();
}
