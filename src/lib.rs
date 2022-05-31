mod activity_info;
mod building;
mod building_job;
mod common;
mod init;
mod jobs;
mod loading;
mod menu;
mod movement;
mod resources;
mod tree;
mod tree_cutting_job;
mod planting_crops;

use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;

use activity_info::ActivityInfoPlugin;
use bevy::app::App;
// #[cfg(debug_assertions)]
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use building_job::BuildingJobPlugin;
use init::{InitPlugin, WorldParams};
use jobs::JobsPlugin;
use movement::MovementPlugin;
use planting_crops::PlantingCropsPlugin;
use resources::ResourcesPlugin;
use tree_cutting_job::TreeCuttingJobPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let world_params = WorldParams {
            size: Vec2::new(800.0, 800.0),
        };
        app.insert_resource(world_params);

        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(MovementPlugin)
            .add_plugin(ResourcesPlugin)
            .add_plugin(JobsPlugin)
            .add_plugin(ActivityInfoPlugin)
            .add_plugin(BuildingJobPlugin)
            .add_plugin(TreeCuttingJobPlugin)
            .add_plugin(PlantingCropsPlugin)
            .add_plugin(InitPlugin);

        // #[cfg(debug_assertions)]
        // {
        //     app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //         .add_plugin(LogDiagnosticsPlugin::default());
        // }
    }
}
