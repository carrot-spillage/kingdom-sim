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
mod timer_plugin;
mod weather;
mod work;

use crate::ambience::{DayNightPlugin, TemperaturePlugin};
use crate::biomes::SoilFertilityLayerPlugin;
use crate::building::{ConstructionPlugin, CreatureConstructingTaskPlugin};
use crate::datetime::GameTimePlugin;
use crate::environment_hud::EnvironmentHudPlugin;
use crate::loading::{BuildingPrefabVec, LoadingPlugin};
use crate::occupy_tiles_plugin::OccupyTilesPlugin;
use crate::plants::bundle::{Germinator, Growing};
use crate::plants::PlantResourceProducer;
use crate::post_processing::PostProcessPlugin;
use crate::quad_tree::QuadTree;
use crate::timer_plugin::TimerPlugin;
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
use create_world::{CreateWorldPlugin, WorldParams};

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

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let tile_side = 16;
        let map_size_factor: u32 = 11; // 2^5 tiles = 512
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
            .add_plugins(YamlAssetPlugin::<PlantPrefabVec>::new(&["plants.yaml"]))
            .add_plugins(YamlAssetPlugin::<ItemPrefabVec>::new(&["items.yaml"]))
            .add_plugins(YamlAssetPlugin::<BuildingPrefabVec>::new(&[
                "buildings.yaml",
            ]))
            .add_plugins(LoadingPlugin)
            // external plugins
            .add_plugins(PanCamPlugin::default())
            .add_plugins(RngPlugin::default().with_rng_seed(12345))
            // game logic plugins
            .add_plugins(GameTimePlugin)
            .add_plugins(CarrierPlugin)
            .add_plugins(CraftingProcessPlugin)
            .add_plugins(CreatureConstructingTaskPlugin)
            // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
            // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
            // .add_plugins(MenuPlugin)
            .add_plugins(TaskPlugin)
            .add_plugins(MovementPlugin)
            .add_plugins(TimerPlugin::<Growing>::new()) // Maybe it doesn't have to come before plugins that use it
            .add_plugins(TimerPlugin::<PlantResourceProducer>::new()) // Maybe it doesn't have to come before plugins that use it
            .add_plugins(TimerPlugin::<Germinator>::new()) // Maybe it doesn't have to come before plugins that use it
            .add_plugins(PlantsPlugin)
            .add_plugins(HarvestingPlugin)
            .add_plugins(ConstructionPlugin)
            .add_plugins(TreeCuttingPlugin)
            .add_plugins(PlantingPlugin)
            .add_plugins(SoilFertilityLayerPlugin { z_offset: 3.0 })
            .add_plugins(CreateWorldPlugin)
            .add_plugins(DayNightPlugin)
            .add_plugins(TemperaturePlugin)
            .add_plugins(PostProcessPlugin)
            .add_plugins(EnvironmentHudPlugin)
            // stuff added for tilemap
            //.set(ImagePlugin::default_nearest())
            .add_plugins(TilemapPlugin)
            .add_plugins(OccupyTilesPlugin);

        // #[cfg(debug_assertions)]
        // {
        //     app.add_plugins(FrameTimeDiagnosticsPlugin::default())
        //         .add_plugins(LogDiagnosticsPlugin::default());
        // }
    }
}
