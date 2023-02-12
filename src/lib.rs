mod building;
mod building_job;
mod common;
mod init;
mod loading;
mod worker_job_tooltip;
// mod menu;
mod monkey_planner;
mod movement;
mod planned_work;

mod crafting_progress;
mod cutting_tree;
mod harvesting;
mod items;
mod planting;
mod planting_crops;
mod plants;
mod skills;
mod stockpile;
mod tasks;
mod work_progress;
use crate::loading::LoadingPlugin;

// use crate::menu::MenuPlugin;

use bevy::app::App;
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_turborand::RngPlugin;
use harvesting::HarvestingPlugin;
use loading::{ItemPrefabVec, PlantPrefabVec};
use planting::PlantingPlugin;

use tasks::TaskPlugin;
use worker_job_tooltip::WorkerJobTooltipPlugin;
// #[cfg(debug_assertions)]
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use building_job::BuildingJobPlugin;
use init::{InitPlugin, WorldParams};

use movement::MovementPlugin;

use cutting_tree::TreeCuttingPlugin;
use planned_work::WorkOnArrivalPlugin;
use plants::PlantsPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    CreatingWorld,
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
            .add_plugin(YamlAssetPlugin::<PlantPrefabVec>::new(&["plants.yaml"]))
            .add_plugin(YamlAssetPlugin::<ItemPrefabVec>::new(&["items.yaml"]))
            .add_plugin(LoadingPlugin)
            .add_plugin(RngPlugin::default())
            // .add_plugin(MenuPlugin)
            .add_plugin(TaskPlugin)
            .add_plugin(MovementPlugin)
            //.add_plugin(JobsPlugin)
            .add_plugin(WorkerJobTooltipPlugin)
            .add_plugin(PlantsPlugin)
            .add_plugin(HarvestingPlugin)
            // .add_plugin(BuildingJobPlugin)
            .add_plugin(TreeCuttingPlugin)
            .add_plugin(PlantingPlugin)
            .add_plugin(WorkOnArrivalPlugin)
            .add_plugin(BuildingJobPlugin)
            .add_plugin(InitPlugin);
        // #[cfg(debug_assertions)]
        // {
        //     app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //         .add_plugin(LogDiagnosticsPlugin::default());
        // }
    }
}
