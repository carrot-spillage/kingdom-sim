use bevy::prelude::{
    App, Assets, Commands, Component, Entity, EventReader, EventWriter, Handle, Image,
    IntoSystemAppConfig, IntoSystemConfigs, OnEnter, OnUpdate, Plugin, Quat, Query, Rect, Res,
    ResMut, Transform, Vec2, With,
};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::sprite::SpriteBundle;
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use noise::{Fbm, Perlin};

use crate::create_world::WorldParams;
use crate::GameState;

use super::air_block::Wind;

#[derive(Component)]
struct CloudChunk {
    position: Vec2,
}

pub fn generate_clouds(size: Vec2) -> Image {
    let fbm = Fbm::<Perlin>::new(0);
    let size = Extent3d {
        depth_or_array_layers: 1,
        width: size.x as u32,
        height: size.y as u32,
    };
    let x = PlaneMapBuilder::<_, 2>::new(&fbm)
        .set_size(size.width as usize, size.height as usize)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build();

    let data: Vec<u8> = x
        .into_iter()
        .flat_map(|x| [63, 63, 63, ((1.0 - x) * 255.0 * 0.25) as u8])
        .collect();
    Image::new(size, TextureDimension::D2, data, TextureFormat::Rgba8Unorm)
}

pub struct CloudPlugin;

impl Plugin for CloudPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RegenerateCloudChunkEvent>()
            .add_system(init.in_schedule(OnEnter(GameState::Playing)))
            .add_systems((move_blocks, generate_image).in_set(OnUpdate(GameState::Playing)));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn move_blocks(
    wind: Res<Wind>,
    mut cloud_chunks: Query<(Entity, &mut CloudChunk)>,
    mut events: EventWriter<RegenerateCloudChunkEvent>,
) {
    let chunk_size = 64.0;
    let shift = wind.direction * wind.speed;
    for (entity, mut cloud_chunk) in cloud_chunks.iter_mut() {
        cloud_chunk.position += shift;
        let index_shift = cloud_chunk.position / chunk_size;
        let index_shift_abs = index_shift.abs();
        let buffer_chunk_count = 2;

        if index_shift_abs.x.abs() > buffer_chunk_count as f32
            || index_shift_abs.y.abs() > buffer_chunk_count as f32
        {
            cloud_chunk.position = -cloud_chunk.position; // move it to the opposite side and generate new pattern
            events.send(RegenerateCloudChunkEvent(entity));
        }
    }
}

struct RegenerateCloudChunkEvent(Entity);

fn generate_image(
    mut events: EventReader<RegenerateCloudChunkEvent>,
    mut cloud_image_handles: Query<&mut Handle<Image>, With<CloudChunk>>,
    world_params: Res<WorldParams>,
    mut images: ResMut<Assets<Image>>,
) {
    for event in &mut events {
        let mut image_handle = cloud_image_handles.get_mut(event.0).unwrap();
        *image_handle = images.set(image_handle.clone(), generate_clouds(world_params.size));
    }
}

fn init(mut images: ResMut<Assets<Image>>, mut commands: Commands, world_params: Res<WorldParams>) {
    let chunk_size = 256;
    let buffer_chunk_count = 2; // todo make it a constant
    let tile_grid = world_params.size / (chunk_size * 2) as f32;
    let cols = tile_grid.x + buffer_chunk_count as f32;
    let rows = tile_grid.y + buffer_chunk_count as f32;
    let image_handle = images.add(Image::default());
    for x in (-cols as u32)..(cols as u32) {
        for y in (-rows as u32)..(rows as u32) {
            commands
                .spawn(SpriteBundle {
                    texture: image_handle.clone(),
                    ..SpriteBundle::default()
                })
                .insert(CloudChunk {
                    position: Vec2::new((x * chunk_size) as f32, (y * chunk_size) as f32),
                });
        }
    }
}

/*
   cloud lifecycle:
   formation,
   growth,
   raining+dissipation

   conditions for formation: hightened evaporation
   looks: some kind of perlin noise overlayed by blocks of different levels of humidity
   conditions for raining: high temperature
*/
