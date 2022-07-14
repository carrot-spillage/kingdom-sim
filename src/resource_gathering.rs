use bevy::{
    math::{Vec2, Vec3},
    prelude::{
        App, Commands, Component, Entity, EventReader, Or, Plugin, Query, Res, SystemSet, With,
        Without,
    },
};

use crate::{
    common::ClaimedBy,
    init::get_random_pos,
    loading::TextureAssets,
    movement::{ArrivedToEntityEvent, MovingToEntity, Position},
    planting_crops::OccupiedArea,
    resources::{
        add_resource_chunk, spawn_resource, CarryingResources, ResourceCarrier, ResourceChunk,
        ResourceChunkAddingResult,
    },
    stockpile::{InStockpile, Stockpile},
    GameState,
};

pub struct ResourceGatheringJobPlugin;

#[derive(Component, Debug)]
pub struct TaskedWithGatheringResources;

#[derive(Component, Debug)]
pub struct LockedOnResourceChunk(pub Entity);

#[derive(Component, Debug)]
pub struct BringingResourcesToStockpile;

impl Plugin for ResourceGatheringJobPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(handle_work)
                .with_system(handle_arrivals)
                .with_system(handle_stockpile_arrivals),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn handle_work(
    mut commands: Commands,
    resource_chunks: Query<
        Entity,
        (
            With<Position>,
            With<ResourceChunk>,
            Without<ClaimedBy>,
            Without<InStockpile>,
        ),
    >,
    query: Query<
        (Entity, Option<&CarryingResources>),
        (
            With<TaskedWithGatheringResources>,
            Without<LockedOnResourceChunk>,
            Without<BringingResourcesToStockpile>,
        ),
    >,
    stockpiles: Query<Entity, With<Stockpile>>,
) {
    for (worker_id, maybe_carrying_resources) in query.iter() {
        println!("Carrier worker: {:?}", worker_id);
        let maybe_nearest_resource_chunk = resource_chunks.iter().next(); // find_nearest_resource(&mut resources);
        if let Some(nearest_resource_chunk) = maybe_nearest_resource_chunk {
            commands
                .entity(nearest_resource_chunk)
                .insert(ClaimedBy(worker_id));
            commands
                .entity(worker_id)
                .insert(LockedOnResourceChunk(nearest_resource_chunk))
                .insert(MovingToEntity {
                    destination_entity: nearest_resource_chunk,
                    sufficient_range: 20.0,
                });
        } else if maybe_carrying_resources.is_some() {
            commands
                .entity(worker_id)
                .insert(BringingResourcesToStockpile)
                .insert(MovingToEntity {
                    sufficient_range: 20.0,
                    destination_entity: stockpiles.iter().next().unwrap(),
                });
        } else {
            // TODO: when we have no resources to pick we should switch to other jobs and then return here eventually
            // commands
            //     .entity(worker_id)
            //     .remove::<TaskedWithGatheringResources>();
        }
    }
}

// fn find_nearest_resource(
//     resources: &mut Query<Entity, (With<Position>, Without<ClaimedBy>)>,
// ) -> Option<(Entity, &ResourceChunk)> {
//     resources.iter().next()
// }

fn handle_arrivals(
    mut commands: Commands,
    mut arrival_events: EventReader<ArrivedToEntityEvent>,
    resources: Query<(Entity, &ResourceChunk)>,
    mut pickers: Query<(
        &LockedOnResourceChunk,
        &ResourceCarrier,
        Option<&mut CarryingResources>,
    )>,
    stockpiles: Query<Entity, With<Stockpile>>,
) {
    for event in arrival_events.iter() {
        let worker_id = event.moving_entity;
        if let Ok((locked_on_resource_chunk, resource_carrier, maybe_carrying_resources)) =
            pickers.get_mut(worker_id)
        {
            let (resource_chunk_id, resource_chunk) =
                resources.get(locked_on_resource_chunk.0).unwrap();
            let current_resource_chunks = maybe_carrying_resources.map_or(vec![], |x| x.0.clone());
            let result = add_resource_chunk(
                &current_resource_chunks,
                resource_chunk,
                resource_carrier.max_volume,
            );

            commands.entity(worker_id).remove::<LockedOnResourceChunk>();
            commands.entity(resource_chunk_id).remove::<ClaimedBy>();

            match result {
                ResourceChunkAddingResult::AddedAll(new_resource_chunks) => {
                    println!("Added all");

                    commands
                        .entity(worker_id)
                        .insert(CarryingResources(new_resource_chunks));

                    commands.entity(resource_chunk_id).despawn();
                }
                ResourceChunkAddingResult::AddedPart(
                    new_resource_chunks,
                    remaining_resource_chunk,
                ) => {
                    commands
                        .entity(worker_id)
                        .insert(CarryingResources(new_resource_chunks));
                    println!(
                        "AddedPart: remaining resource chunk: {:?}",
                        remaining_resource_chunk
                    );
                    commands
                        .entity(worker_id)
                        .insert(BringingResourcesToStockpile)
                        .insert(MovingToEntity {
                            sufficient_range: 20.0,
                            destination_entity: stockpiles.iter().next().unwrap(),
                        });

                    commands
                        .entity(resource_chunk_id)
                        .insert(remaining_resource_chunk);
                }
                ResourceChunkAddingResult::AddedNone => {
                    println!("Added none");

                    commands
                        .entity(worker_id)
                        .insert(BringingResourcesToStockpile)
                        .insert(MovingToEntity {
                            sufficient_range: 20.0,
                            destination_entity: stockpiles.iter().next().unwrap(),
                        });
                }
            }
        }
    }
}

fn handle_stockpile_arrivals(
    mut commands: Commands,
    mut arrival_events: EventReader<ArrivedToEntityEvent>,
    mut carriers: Query<&mut CarryingResources, With<BringingResourcesToStockpile>>,
    stockpiles: Query<(&OccupiedArea, &Position), With<Stockpile>>,
    textures: Res<TextureAssets>,
) {
    for event in arrival_events.iter() {
        let worker_id = event.moving_entity;
        if let Ok(carrying_resources) = carriers.get_mut(worker_id) {
            let stockpile_id = event.destination_entity;
            commands
                .entity(worker_id)
                .remove::<BringingResourcesToStockpile>()
                .remove::<CarryingResources>();
            let (stockpile_area, stockpile_position) = stockpiles.get(stockpile_id).unwrap();

            add_resource_chunks_to_stockpile(
                &mut commands,
                stockpile_position.0,
                stockpile_area.0,
                &carrying_resources.0,
                &textures,
            );
        }
    }
}

fn add_resource_chunks_to_stockpile(
    commands: &mut Commands,
    stockpile_position: Vec3,
    stockpile_area: Vec2,
    carrying_resources: &Vec<ResourceChunk>,
    textures: &Res<TextureAssets>,
) {
    // TODO: for now we inefficiently drop resource chunks without merging with one of the existing chunks
    for resource_chunk in carrying_resources {
        let position = get_random_pos(stockpile_position.truncate(), stockpile_area / 2.0);
        spawn_resource(commands, textures, *resource_chunk, position, true)
    }
}

pub fn plan_resource_gathering(commands: &mut Commands, worker_id: Entity) {
    commands
        .entity(worker_id)
        .insert(TaskedWithGatheringResources);
}
