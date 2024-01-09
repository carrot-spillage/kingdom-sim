use bevy::{
    asset::{},
    ecs::query::Changed,
    prelude::{
        in_state, App, Color, Component, IntoSystemConfigs, OnEnter, Plugin, Query, Update,
    },
    render::texture::Image,
};

use bevy_ecs_tilemap::{
    prelude::{},
    tiles::{TileColor},
};
use bevy_turborand::{DelegatedRng};
use noise::{
    utils::{NoiseMapBuilder, PlaneMapBuilder},
    Fbm, Perlin,
};

use crate::{common::TilemapZOffset, create_world::WorldParams, tilemap_utils::new_tilemap_bundle};

use crate::{loading::TextureAssets, GameState};

use super::{tile_image::generate_image, overlay_tilemap::create_tilemap};

#[derive(Component)]
pub struct SoilFertility(pub f32); // 0..1

pub struct SoilFertilityLayerPlugin {
    pub z_offset: f32,
}


#[derive(Component)]
pub struct SoilFertilityTilemap;

impl Plugin for SoilFertilityLayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TilemapZOffset(self.z_offset))
            .add_systems(OnEnter(GameState::CreatingWorld), create_tilemap)
            .add_systems(Update, update_tiles.run_if(in_state(GameState::Playing)));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn update_tiles(mut tiles: Query<(&SoilFertility, &mut TileColor), Changed<SoilFertility>>) {
    // TODO: querying this has a perf impact
    for (fertility, mut tile_color) in &mut tiles {
        tile_color.0.set_a(fertility.0);
    }
}

pub fn generate_fertility(seed: u32, width: u32, height: u32) -> Vec<(u32, u32, f64)> {
    let fbm = Fbm::<Perlin>::new(seed);

    PlaneMapBuilder::<_, 2>::new(&fbm)
        .set_size(width as usize, height as usize)
        .set_x_bounds(-2.0, 2.0)
        .set_y_bounds(-2.0, 2.0)
        .build()
        .iter()
        .enumerate()
        .map(|(index, value)| (index as u32 % width, index as u32 / width, *value))
        .collect()
}
