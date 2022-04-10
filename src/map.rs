use crate::{loading::TextureAssets, GameState};

use bevy::{
    app::Plugin,
    asset::Assets,
    ecs::{
        schedule::SystemSet,
        system::{Commands, Res, ResMut},
    },
    render::{camera::OrthographicCameraBundle, render_resource::TextureUsages, texture::Image},
    transform::components::{GlobalTransform, Transform},
};
use bevy_ecs_tilemap::{
    ChunkSize, LayerBuilder, LayerSettings, Map, MapQuery, MapSize, TextureSize, TileBundle,
    TileSize, TilemapPlugin,
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(TilemapPlugin).add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_camera)
                .with_system(spawn_map),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_map(
    mut commands: Commands,
    mut map_query: MapQuery,
    textures: Res<TextureAssets>,
    mut assets: ResMut<Assets<Image>>,
) {
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let (mut layer_builder, _) = LayerBuilder::new(
        &mut commands,
        LayerSettings::new(
            MapSize(2, 2),
            ChunkSize(32, 32),
            TileSize(22.0, 22.0),
            TextureSize(22.0, 22.0),
        ),
        0u16,
        0u16,
    );

    layer_builder.set_all(TileBundle::default());

    let material_handle = textures.grass_tile.clone();
    assets
        .get_mut(material_handle.clone())
        .unwrap()
        .texture_descriptor
        .usage = TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_SRC | TextureUsages::COPY_DST;

    let layer_entity = map_query.build_layer(&mut commands, layer_builder, material_handle);

    map.add_layer(&mut commands, 0u16, layer_entity);

    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-704.0, -704.0, 0.0))
        .insert(GlobalTransform::default());
}
