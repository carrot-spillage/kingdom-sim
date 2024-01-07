use bevy::{math::Vec2, prelude::Res};
use bevy_ecs_tilemap::{
    prelude::{
        get_tilemap_center_transform, IsoCoordSystem, TilemapSize, TilemapTexture, TilemapTileSize,
        TilemapType,
    },
    tiles::TileStorage,
    TilemapBundle,
};

use crate::common::TilemapZOffset;

pub fn new_tilemap_bundle(
    half_max_isometric_z: f32,
    z_offset: Res<TilemapZOffset>,
    texture: TilemapTexture,
    tile_side: f32,
    world_size: Vec2,
) -> TilemapBundle {
    let total_z_offset: f32 = half_max_isometric_z * 2.0 + z_offset.0;

    let tile_size = TilemapTileSize {
        x: tile_side * 2.0,
        y: tile_side,
    };

    println!("tile_size {:?}", tile_size);

    let grid_size = tile_size.into();
    let map_type = TilemapType::Isometric(IsoCoordSystem::Diamond);

    let tilemap_size = TilemapSize {
        x: (world_size.x / tile_side) as u32,
        y: (world_size.y / tile_side) as u32,
    };

    println!("tilemap_size {:?}", tilemap_size);

    TilemapBundle {
        tile_size,
        grid_size,
        map_type,
        size: tilemap_size,
        texture,
        storage: TileStorage::empty(tilemap_size),
        transform: get_tilemap_center_transform(
            &tilemap_size,
            &grid_size,
            &map_type,
            total_z_offset,
        ), // TODO: create proper structure for UI layers
        ..Default::default()
    }
}
