use bevy::{
    asset::Assets,
    ecs::system::ResMut,
    prelude::{Commands, Component, Res},
    render::{color::Color, texture::Image},
};
use bevy_ecs_tilemap::{
    prelude::{
        get_tilemap_center_transform, IsoCoordSystem, TilemapId, TilemapSize, TilemapTexture,
        TilemapTileSize, TilemapType,
    },
    tiles::{TileBundle, TilePos, TileStorage},
    TilemapBundle,
};

use crate::{biomes::generate_tile_image, create_world::WorldParams};

#[derive(Component)]
pub struct LandTilemap;

pub fn create_land_tilemap(
    commands: &mut Commands,
    world_params: &Res<WorldParams>,
    assets: &mut ResMut<Assets<Image>>,
) {
    println!("Creating land tilemap");

    let tile_size = TilemapTileSize {
        x: world_params.tile_side * 2.0,
        y: world_params.tile_side,
    };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Isometric(IsoCoordSystem::Diamond);

    let tilemap_entity = commands.spawn(LandTilemap).id();
    let map_size = TilemapSize {
        x: (world_params.size.x / world_params.tile_side) as u32,
        y: (world_params.size.y / world_params.tile_side) as u32,
    };
    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_side = world_params.tile_side * 16.0;

    let image = generate_tile_image(assets, tile_side as u32, Color::SEA_GREEN);

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(image.clone()),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}
