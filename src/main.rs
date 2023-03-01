// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod quad_tree;

use bevy::prelude::{App, ClearColor, Color, Msaa, PluginGroup, WindowDescriptor};
use bevy::window::WindowPlugin;
use bevy::DefaultPlugins;
use kingdom_sim::GamePlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 800.,
                height: 600.,
                title: "kingdom_sim".to_string(),
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(GamePlugin)
        .run();
}
