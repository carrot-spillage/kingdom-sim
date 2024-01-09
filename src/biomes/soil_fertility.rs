use bevy::{
    asset::{Assets, Handle},
    ecs::query::Changed,
    math::{UVec2, Vec2},
    prelude::{
        in_state, App, Color, Commands, Component, IntoSystemConfigs, OnEnter, Plugin, Query, Res,
        ResMut, Update,
    },
    render::texture::Image,
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
            .add_systems(OnEnter(GameState::CreatingWorld), create_tilemap)
            .add_systems(Update, update_tiles.run_if(in_state(GameState::Playing)));
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
    assets: ResMut<Assets<Image>>,
) {
    let tile_side = world_params.tile_side * 16.0;

    let image = setup_image(assets, tile_side as u32);
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

fn generate_overlay(
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

use image::{DynamicImage, ImageBuffer};

/// This function builds your image, you can use any pixel format you like
fn make_rhombus_tile(
    canvas_size: UVec2,
    color: image::Rgba<u8>,
    border_width: u32,
    border_color: image::Rgba<u8>,
) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    println!("canvas_size: {:?}", canvas_size);
    let mut image = ImageBuffer::new(canvas_size.x, canvas_size.y);

    let f_canvas_size = canvas_size.as_vec2();
    let mid = f_canvas_size / 2.0;
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        if is_in_rhombus(Vec2::new(x as f32, y as f32), f_canvas_size, mid) {
            *pixel = color;
        }
    }

    image
}

fn is_in_rhombus(point: Vec2, size: Vec2, mid: Vec2) -> bool {
    let point = point - mid;
    let size = size - mid;

    let point = point;
    let size = size;

    let point = (point / size).abs();

    point.x + point.y <= 1.0
}
fn setup_image(mut images: ResMut<Assets<Image>>, tile_side: u32) -> Handle<Image> {
    let image: ImageBuffer<image::Rgba<u8>, Vec<u8>> = make_rhombus_tile(
        UVec2::new(tile_side * 2, tile_side),
        image::Rgba([255, 255, 255, 255]),
        0,
        image::Rgba([255, 255, 255, 255]),
    );
    // This does 3 things in one line:
    // 1. Create a DynamicImage from our ImageBuffer
    // 2. Convert that to an ImageBuffer<Rgba<u8>, _>
    // 3. Convert it back into a DynamicImage
    // You can skip steps 2 and 3 here if your image is built in Rgba<u8>
    let dynamic_image = DynamicImage::from(image).to_rgba8().into();

    // Now add it to Bevy!
    return images.add(Image::from_dynamic(dynamic_image, true));
    // Then spawn the sprite, or whatever else you'd like to do with it
}
