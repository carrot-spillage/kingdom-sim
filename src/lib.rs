mod building;
mod building_job;
mod common;
mod init;
mod loading;
// mod menu;
mod monkey_planner;
mod movement;
mod planned_work;

mod crafting_progress;
mod creature;
mod cutting_tree;
mod harvesting;
mod items;
mod planting;
mod plants;
mod quad_tree;
mod skills;
mod tasks;
mod work_progress;

use crate::quad_tree::QuadTree;
use crate::{init::OccupyTilesPlugin, loading::LoadingPlugin};
// use crate::menu::MenuPlugin;

use bevy::app::App;
use bevy::prelude::system_adapter::new;
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_turborand::RngPlugin;
use harvesting::HarvestingPlugin;
use loading::{ItemPrefabVec, PlantPrefabVec};
use planting::PlantingPlugin;

use tasks::TaskPlugin;
// #[cfg(debug_assertions)]
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use building_job::BuildingJobPlugin;
use init::{InitPlugin, WorldParams};

use movement::MovementPlugin;

use creature::CarrierPlugin;
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
        let tile_side = 16;
        let map_size_factor: u32 = 5; // 2^5 tiles = 512
        let side = (2_u32.pow(map_size_factor) * tile_side) as f32;
        let size = Vec2::new(side, side);
        let world_params = WorldParams {
            side,
            size,
            tile_side: tile_side as f32,
            half_max_isometric_z: side + 10.0, // 10 z layers to cover special cases
        };
        println!("World params {:?}", world_params);
        app.insert_resource(world_params);
        app.insert_resource(QuadTree::<Entity>::new(
            Rect::from_corners(Vec2::ZERO, size),
            map_size_factor,
        ));
        app.add_state(GameState::Loading)
            .add_plugin(YamlAssetPlugin::<PlantPrefabVec>::new(&["plants.yaml"]))
            .add_plugin(YamlAssetPlugin::<ItemPrefabVec>::new(&["items.yaml"]))
            .add_plugin(LoadingPlugin)
            .add_plugin(RngPlugin::default().with_rng_seed(12345))
            .add_plugin(CarrierPlugin)
            .add_plugin(OccupyTilesPlugin)
            // .add_plugin(MenuPlugin)
            .add_plugin(TaskPlugin)
            .add_plugin(MovementPlugin)
            .add_plugin(PlantsPlugin)
            .add_plugin(HarvestingPlugin)
            // .add_plugin(BuildingJobPlugin)
            .add_plugin(TreeCuttingPlugin)
            .add_plugin(PlantingPlugin)
            .add_plugin(WorkOnArrivalPlugin)
            .add_plugin(BuildingJobPlugin)
            .add_plugin(InitPlugin)
            // stuff added for tilemap
            //.set(ImagePlugin::default_nearest())
            .add_plugin(TilemapPlugin);
        // .add_startup_system(startup)
        // .add_system(helpers::camera::movement);
        // #[cfg(debug_assertions)]
        // {
        //     app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //         .add_plugin(LogDiagnosticsPlugin::default());
        // }
    }
}
