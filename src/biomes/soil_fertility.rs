use bevy::prelude::{
    App, Color, Commands, Component, IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnUpdate,
    Plugin, Query, Res, ResMut,
};
use bevy_ecs_tilemap::{
    prelude::{TilemapId, TilemapTexture},
    tiles::{TileBundle, TileColor, TilePos},
};
use bevy_turborand::{DelegatedRng, GlobalRng};
use noise::{
    utils::{NoiseMapBuilder, PlaneMapBuilder},
    Fbm, Perlin,
};

use crate::{common::TilemapZOffset, create_world::WorldParams, tilemap_utils::new_tilemap_bundle};

use crate::{loading::TextureAssets, GameState};

#[derive(Component)]
pub struct SoilFertility(pub f32); // 0..1

pub struct SoilFertilityLayerPlugin {
    pub z_offset: f32,
}

static BASE_COLOR: Color = Color::rgb(0.55, 0.27, 0.07);

#[derive(Component)]
pub struct SoilFertilityTilemap;

impl Plugin for SoilFertilityLayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TilemapZOffset(self.z_offset))
            .add_system(create_tilemap.in_schedule(OnEnter(GameState::CreatingWorld)))
            .add_system(update_tiles.in_set(OnUpdate(GameState::Playing)));
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
    let mut tilemap_bundle = new_tilemap_bundle(
        world_params,
        z_offset,
        TilemapTexture::Single(textures.blank_tile.clone()),
    );
    let tile_storage = &mut tilemap_bundle.storage;
    let map_size = tilemap_bundle.size;
    let tilemap_entity = commands.spawn(SoilFertilityTilemap).id();

    for (x, y, fertility) in
        generate_fertility(global_rng.u32(0..=u32::MAX), map_size.x, map_size.y)
    {
        let tile_pos = TilePos { x, y };
        let tile_entity = commands
            .spawn((
                TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    color: TileColor(BASE_COLOR),
                    ..Default::default()
                },
                SoilFertility(fertility as f32),
            ))
            .id();
        tile_storage.set(&tile_pos, tile_entity);
    }

    commands.entity(tilemap_entity).insert(tilemap_bundle);
}

fn update_tiles(mut tiles: Query<(&SoilFertility, &mut TileColor)>) {
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
