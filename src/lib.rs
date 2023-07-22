mod building;
mod common;
mod create_world;
mod loading;
// mod menu;
mod ambience;
mod biomes;
mod creature;
mod cutting_tree;
mod datetime;
mod movement;
mod post_processing;

mod environment_hud;
mod harvesting;
mod items;
mod land_tilemap;
mod occupy_tiles_plugin;
mod planting;
mod plants;
mod quad_tree;
mod tasks;
mod tilemap_utils;
mod weather;
mod work;

use crate::ambience::DayNightPlugin;
use crate::biomes::SoilFertilityLayerPlugin;
use crate::building::{ConstructionPlugin, CreatureConstructingTaskPlugin};
use crate::datetime::GameTimePlugin;
use crate::environment_hud::EnvironmentHudPlugin;
use crate::loading::{BuildingPrefabVec, LoadingPlugin};
use crate::occupy_tiles_plugin::OccupyTilesPlugin;
use crate::post_processing::PostProcessPlugin;
use crate::quad_tree::QuadTree;
use crate::work::CraftingProcessPlugin;
// use crate::menu::MenuPlugin;

use bevy::app::App;
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_pancam::PanCamPlugin;
use bevy_turborand::prelude::RngPlugin;
use harvesting::HarvestingPlugin;
use loading::{ItemPrefabVec, PlantPrefabVec};
use planting::PlantingPlugin;
use tasks::TaskPlugin;
// #[cfg(debug_assertions)]
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use create_world::{InitPlugin, WorldParams};

use creature::CarrierPlugin;
use cutting_tree::TreeCuttingPlugin;
use movement::MovementPlugin;
use plants::PlantsPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, PartialEq, Eq, Debug, Hash, Clone, Default)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
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
        let map_size_factor: u32 = 7; // 2^5 tiles = 512
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
            Rect::from_corners(-size / 2.0, size / 2.0),
            map_size_factor,
        ));
        app.add_state::<GameState>()
            .add_plugin(YamlAssetPlugin::<PlantPrefabVec>::new(&["plants.yaml"]))
            .add_plugin(YamlAssetPlugin::<ItemPrefabVec>::new(&["items.yaml"]))
            .add_plugin(YamlAssetPlugin::<BuildingPrefabVec>::new(&[
                "buildings.yaml",
            ]))
            .add_plugin(LoadingPlugin)
            // external plugins
            .add_plugin(PanCamPlugin::default())
            .add_plugin(RngPlugin::default().with_rng_seed(12345))
            // game logic plugins
            .add_plugin(GameTimePlugin)
            .add_plugin(CarrierPlugin)
            .add_plugin(CraftingProcessPlugin)
            .add_plugin(CreatureConstructingTaskPlugin)
            // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
            // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
            // .add_plugin(MenuPlugin)
            .add_plugin(TaskPlugin)
            .add_plugin(MovementPlugin)
            // .add_plugin(CloudPlugin)
            .add_plugin(PlantsPlugin)
            .add_plugin(HarvestingPlugin)
            .add_plugin(ConstructionPlugin)
            .add_plugin(TreeCuttingPlugin)
            .add_plugin(PlantingPlugin)
            .add_plugin(SoilFertilityLayerPlugin { z_offset: 3.0 })
            .add_plugin(InitPlugin)
            .add_plugin(DayNightPlugin)
            .add_plugin(PostProcessPlugin)
            .add_plugin(EnvironmentHudPlugin)
            // stuff added for tilemap
            //.set(ImagePlugin::default_nearest())
            .add_plugin(TilemapPlugin)
            .add_plugin(OccupyTilesPlugin);
        // .add_startup_system(startup)
        // .add_system(helpers::camera::movement);
        // #[cfg(debug_assertions)]
        // {
        //     app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //         .add_plugin(LogDiagnosticsPlugin::default());
        // }
    }
}
