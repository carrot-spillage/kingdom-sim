mod worker_job_tooltip;
mod building;
mod building_job;
mod common;
mod init;
mod loading;
// mod menu;
mod monkey_planner;
mod movement;
mod planned_work;
mod resource_gathering;

mod resources;
mod skills;
mod tree;
mod tree_cutting;
mod work_progress;
mod stockpile;
mod crafting_progress;
mod items;
mod plants;
mod planting;
mod planting_crops; // deprecated
use crate::loading::LoadingPlugin;
// use crate::menu::MenuPlugin;

use planting::PlantingPlugin;
use worker_job_tooltip::WorkerJobTooltipPlugin;
use bevy::app::App;
// #[cfg(debug_assertions)]
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use building_job::BuildingJobPlugin;
use init::{InitPlugin, WorldParams};

use movement::MovementPlugin;

use planned_work::WorkOnArrivalPlugin;
use resource_gathering::ResourceGatheringJobPlugin;
use resources::ResourcesPlugin;
use tree_cutting::TreeCuttingPlugin;

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

pub struct Dummy;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let world_params = WorldParams {
            size: Vec2::new(800.0, 800.0),
        };
        app.insert_resource(world_params);

        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            // .add_plugin(MenuPlugin)
            .add_plugin(MovementPlugin)
            .add_plugin(ResourcesPlugin)
            //.add_plugin(JobsPlugin)
            .add_plugin(WorkerJobTooltipPlugin)
            // .add_plugin(BuildingJobPlugin)
            .add_plugin(TreeCuttingPlugin)
            .add_plugin(PlantingPlugin)
            .add_plugin(InitPlugin)
            .add_plugin(WorkOnArrivalPlugin)
            .add_plugin(BuildingJobPlugin)
            .add_plugin(ResourceGatheringJobPlugin);

        // #[cfg(debug_assertions)]
        // {
        //     app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //         .add_plugin(LogDiagnosticsPlugin::default());
        // }
    }
}
