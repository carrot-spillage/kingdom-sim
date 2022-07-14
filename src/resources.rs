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
    stockpile::InStockpile,
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

#[derive(Component)]
pub struct CarryingResources(pub Vec<ResourceChunk>);

#[derive(Component)]
pub struct ResourceCarrier {
    pub max_volume: usize,
}

pub struct BreaksIntoResourcesEvent(pub Entity);

pub struct ResourcesPlugin;

fn resource_volume_per_unit(resource_kind: ResourceKind) -> usize {
    match resource_kind {
        ResourceKind::Wood => 20,
    }
}

pub fn max_resource_units_from_chunk(chunk: &ResourceChunk, available_volume: usize) -> usize {
    (available_volume / resource_volume_per_unit(chunk.kind)).min(chunk.quantity)
}

pub enum ResourceChunkAddingResult {
    AddedAll(Vec<ResourceChunk>),
    AddedPart(Vec<ResourceChunk>, ResourceChunk),
    AddedNone,
}

pub fn add_resource_chunk(
    resource_chunks: &Vec<ResourceChunk>,
    added_resource_chunk: &ResourceChunk,
    max_volume: usize,
) -> ResourceChunkAddingResult {
    let current_volume = resource_chunks
        .iter()
        .map(|x| resource_volume_per_unit(x.kind) * x.quantity)
        .sum::<usize>();
    let available_volume = max_volume - current_volume;
    let quantity_to_add = max_resource_units_from_chunk(added_resource_chunk, available_volume);
    if quantity_to_add == 0 {
        ResourceChunkAddingResult::AddedNone
    } else if quantity_to_add == added_resource_chunk.quantity {
        let mut new_resource_chunks = resource_chunks.clone();
        new_resource_chunks.push(*added_resource_chunk);
        ResourceChunkAddingResult::AddedAll(new_resource_chunks)
    } else {
        let mut new_resource_chunks = resource_chunks.clone();
        new_resource_chunks.push(ResourceChunk {
            kind: added_resource_chunk.kind,
            quantity: quantity_to_add,
        });
        ResourceChunkAddingResult::AddedPart(
            new_resource_chunks,
            ResourceChunk {
                kind: added_resource_chunk.kind,
                quantity: added_resource_chunk.quantity - quantity_to_add,
            },
        )
    }
}

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
            spawn_resource(&mut commands, &textures, *resource, *position, false);
        }
        commands.entity(entity).despawn();
    }
}

pub fn spawn_resource(
    commands: &mut Commands,
    textures: &Res<TextureAssets>,
    resource_chunk: ResourceChunk,
    position: Vec3,
    is_in_stockpile: bool,
) {
    println!("Spawning resource");
    let resource_id = commands
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
        })
        .id();

    if is_in_stockpile {
        commands.entity(resource_id).insert(InStockpile);
    }
}
