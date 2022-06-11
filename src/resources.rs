use bevy::{
    math::Vec3,
    prelude::{
        App, Commands, Component, Entity, EventReader, Plugin, Query, Res, SystemSet, Transform,
    },
    sprite::SpriteBundle,
};

use crate::{
    loading::TextureAssets,
    movement::{hack_3d_position_to_2d, Position},
    GameState,
};

#[derive(Debug, Clone, Copy)]
pub enum ResourceKind {
    Wood,
}

#[derive(Clone, Copy, Component, Debug)]
pub struct ResourceChunk {
    pub kind: ResourceKind,
    pub quantity: usize,
}

#[derive(Component)]
pub struct BreaksIntoResources(pub Vec<ResourceChunk>);

pub struct BreaksIntoResourcesEvent(pub Entity);

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BreaksIntoResourcesEvent>().add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(break_into_resources),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn break_into_resources(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut events: EventReader<BreaksIntoResourcesEvent>,
    breakables: Query<(&Position, &BreaksIntoResources)>,
) {
    for event in events.iter() {
        let entity = event.0;
        let (Position(position), BreaksIntoResources(resources)) = breakables.get(entity).unwrap();
        println!("Breaking {:?} into resources  {:?}", entity, resources);

        for resource in resources {
            spawn_resource(&mut commands, &textures, *resource, *position);
        }
        commands.entity(entity).despawn();
    }
}

fn spawn_resource(
    commands: &mut Commands,
    textures: &Res<TextureAssets>,
    resource_chunk: ResourceChunk,
    position: Vec3,
) {
    println!("Spawning resource");
    commands
        .spawn()
        .insert(Position(position))
        .insert(resource_chunk)
        .insert_bundle(SpriteBundle {
            texture: textures.logs.clone(),
            transform: Transform {
                translation: hack_3d_position_to_2d(position),
                scale: Vec3::new(0.3, 0.3, 1.0),
                ..Transform::default()
            },
            ..Default::default()
        });
}
