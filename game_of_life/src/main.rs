use bevy::prelude::*;
use ui::MainMenuPlugin;
use input::InputPlugin;
use simulation::SimulationPlugin;

const GRID_SIZE: i32 = 100;

mod ui;
mod input;
mod simulation;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{ 
            window: WindowDescriptor {
                width: 1024f32,
                height: 720f32,
                title: String::from("Game Of Life"),
                ..default()
            },
            ..default()
        }))
        .add_plugin(MainMenuPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(SimulationPlugin)
        .run();
}
