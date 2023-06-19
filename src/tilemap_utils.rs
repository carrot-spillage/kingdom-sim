use bevy::prelude::Res;
use bevy_ecs_tilemap::{
    prelude::{
        get_tilemap_center_transform, IsoCoordSystem, TilemapSize, TilemapTexture, TilemapTileSize,
        TilemapType,
    },
    tiles::TileStorage,
    TilemapBundle,
};

use crate::{common::TilemapZOffset, create_world::WorldParams};

pub fn new_tilemap_bundle(
    world_params: Res<'_, WorldParams>,
    z_offset: Res<'_, TilemapZOffset>,
    texture: TilemapTexture,
) -> TilemapBundle {
    let total_z_offset = world_params.half_max_isometric_z * 2.0 + z_offset.0;

    let tile_size = TilemapTileSize {
        x: world_params.tile_side * 2.0,
        y: world_params.tile_side,
    };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Isometric(IsoCoordSystem::Diamond);

    let size = TilemapSize {
        x: (world_params.size.x / world_params.tile_side) as u32,
        y: (world_params.size.y / world_params.tile_side) as u32,
    };

    TilemapBundle {
        tile_size,
        grid_size,
        map_type,
        size,
        texture,
        storage: TileStorage::empty(size),
        transform: get_tilemap_center_transform(&size, &grid_size, &map_type, total_z_offset), // TODO: create proper structure for UI layers
        ..Default::default()
    }
}
