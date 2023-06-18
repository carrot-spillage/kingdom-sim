use bevy::prelude::{
    App, Color, Commands, Component, IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnUpdate,
    Plugin, Query, Res, ResMut, With,
};
use bevy_ecs_tilemap::{
    prelude::{
        get_tilemap_center_transform, IsoCoordSystem, TilemapId, TilemapSize, TilemapTexture,
        TilemapTileSize, TilemapType,
    },
    tiles::{TileBundle, TileColor, TilePos, TileStorage},
    TilemapBundle,
};
use bevy_turborand::{DelegatedRng, GlobalRng};

use crate::{common::TilemapZOffset, create_world::WorldParams};

use crate::{loading::TextureAssets, GameState};

pub struct WeatherUIPlugin {
    pub z_offset: f32,
}

#[derive(Component)]
pub struct WeatherTilemap;

impl Plugin for WeatherUIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TilemapZOffset(self.z_offset))
            .add_system(create_tilemap.in_schedule(OnEnter(GameState::CreatingWorld)))
            .add_system(update_tiles_with_humidity.in_set(OnUpdate(GameState::Playing)));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn create_tilemap(
    mut commands: Commands,
    world_params: Res<WorldParams>,
    textures: Res<TextureAssets>,
    mut global_rng: ResMut<GlobalRng>,
    z_offset: Res<TilemapZOffset>,
) {
    let total_z_offset = world_params.half_max_isometric_z * 2.0 + z_offset.0;

    let tile_size = TilemapTileSize {
        x: world_params.tile_side * 2.0,
        y: world_params.tile_side,
    };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Isometric(IsoCoordSystem::Diamond);

    let tilemap_entity = commands.spawn(WeatherTilemap).id();
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
                    color: TileColor(Color::BLUE.with_a(global_rng.f32_normalized())),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(textures.blank_tile.clone()),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, total_z_offset), // TODO: create proper structure for UI layers
        ..Default::default()
    });
}

fn update_tiles_with_humidity(
    mut commands: Commands,
    grids: Query<&TileStorage, With<WeatherTilemap>>,
) {
    let tile_storage = grids.single();

    let map_size = tile_storage.size;
    for x in 0..map_size.x / 2 {
        for y in 0..map_size.y / 2 {
            let tile = tile_storage.get(&TilePos { x, y }).unwrap();
            commands
                .entity(tile)
                .insert(TileColor(Color::BLUE.with_a(0.3)));
        }
    }
}
