// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod quad_tree;

use bevy::prelude::{App, ClearColor, Color, Msaa};
use bevy::DefaultPlugins;
use kingdom_sim::GamePlugin;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
