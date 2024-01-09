use bevy_ecs_tilemap::prelude::TilemapId;

use bevy_ecs_tilemap::tiles::TileBundle;

use bevy_ecs_tilemap::tiles::TilePos;

use bevy_ecs_tilemap::tiles::TileColor;
use bevy_turborand::DelegatedRng;

use super::soil_fertility::generate_fertility;
use super::soil_fertility::SoilFertilityTilemap;
use super::tile_image::generate_image;
use super::SoilFertility;

use bevy_ecs_tilemap::prelude::TilemapTexture;

use crate::tilemap_utils::new_tilemap_bundle;

use bevy::asset::Handle;

use bevy::prelude::Color;

use bevy::render::texture::Image;

use bevy::asset::Assets;

use crate::common::TilemapZOffset;

use bevy_turborand::GlobalRng;

use bevy::prelude::ResMut;

use crate::loading::TextureAssets;

use crate::create_world::WorldParams;

use bevy::prelude::Res;

use bevy::prelude::Commands;

static BASE_COLOR: Color = Color::rgb(0.55, 0.27, 0.07);

pub(crate) fn create_tilemap(
    mut commands: Commands,
    world_params: Res<WorldParams>,
    textures: Res<TextureAssets>,
    mut global_rng: ResMut<GlobalRng>,
    z_offset: Res<TilemapZOffset>,
    assets: ResMut<Assets<Image>>,
) {
    let tile_side = world_params.tile_side * 16.0;

    let image = generate_image(assets, tile_side as u32);
    generate_overlay(
        BASE_COLOR,
        world_params,
        &mut commands,
        &mut global_rng,
        z_offset,
        image,
        tile_side,
    );
}

pub(crate) fn generate_overlay(
    overlay_color: Color,
    world_params: Res<WorldParams>,
    commands: &mut Commands<'_, '_>,
    global_rng: &mut ResMut<GlobalRng>,
    z_offset: Res<TilemapZOffset>,
    tile_texture: Handle<Image>,
    tile_side: f32,
) {
    let mut tilemap_bundle = new_tilemap_bundle(
        world_params.half_max_isometric_z,
        z_offset,
        TilemapTexture::Single(tile_texture),
        tile_side,
        world_params.size,
    );
    let tile_storage = &mut tilemap_bundle.storage;

    let tilemap_entity = commands.spawn(SoilFertilityTilemap).id();

    let overlay_map_size = tilemap_bundle.size;

    let fertility_map = generate_fertility(
        global_rng.u32(0..=u32::MAX),
        overlay_map_size.x,
        overlay_map_size.y,
    );

    let tile_color = TileColor(overlay_color);
    for x in 0..overlay_map_size.x {
        for y in 0..overlay_map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        color: tile_color,
                        ..Default::default()
                    },
                    SoilFertility(fertility_map[(x + y * (overlay_map_size.x)) as usize].2 as f32),
                ))
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    commands.entity(tilemap_entity).insert(tilemap_bundle);
}
