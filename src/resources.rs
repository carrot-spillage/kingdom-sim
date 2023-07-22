use bevy::{
    math::Vec3,
    prelude::{
        App, Commands, Component, Entity, Event, EventReader, Plugin, Query, Res, SystemSet,
        Transform,
    },
    sprite::SpriteBundle,
};

use crate::{
    loading::TextureAssets,
    movement::{hack_3d_position_to_2d, Position},
    stockpile::InStockpile,
    GameState,
};

#[derive(Clone, Copy, Component, Debug)]
pub struct ResourceChunk {
    pub kind: ResourceKind,
    pub quantity: u32,
}

#[derive(Component)]
pub struct BreaksIntoResources(pub Vec<ResourceChunk>);

#[derive(Component)]
pub struct CarryingResources(pub Vec<ResourceChunk>);

#[derive(Component)]
pub struct ResourceCarrier {
    pub max_volume: u32,
}

#[derive(Event)]
pub struct BreaksIntoResourcesEvent(pub Entity);

pub struct ResourcesPlugin;

fn resource_volume_per_unit(resource_kind: ResourceKind) -> u32 {
    match resource_kind {
        ResourceKind::Wood => 20,
    }
}

pub fn max_resource_units_from_chunk(chunk: &ResourceChunk, available_volume: u32) -> u32 {
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
    max_volume: u32,
) -> ResourceChunkAddingResult {
    let current_volume = resource_chunks
        .iter()
        .map(|x| resource_volume_per_unit(x.kind) * x.quantity)
        .sum::<u32>();
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
