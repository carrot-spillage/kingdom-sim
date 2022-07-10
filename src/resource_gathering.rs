use bevy::prelude::{
    App, Commands, Component, Entity, EventReader, Plugin, Query, SystemSet, With, Without,
};

use crate::{
    common::ClaimedBy,
    movement::{ArrivedToEntityEvent, MovingToEntity, Position},
    resources::{CarryingResources, ResourceChunk},
    GameState,
};

static JOB_NAME: &'static str = "ResourceGathering";

pub struct ResourceGatheringJobPlugin;

#[derive(Component, Debug)]
pub struct TaskedWithGatheringResources;

#[derive(Component, Debug)]
pub struct LockedOnResource(pub Entity);

impl Plugin for ResourceGatheringJobPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(handle_work)
                .with_system(handle_arrivals),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn handle_work(
    mut commands: Commands,
    query: Query<
        Entity,
        (
            With<TaskedWithGatheringResources>,
            Without<LockedOnResource>,
        ),
    >,
    mut resources: Query<Entity, (With<ResourceChunk>, With<Position>, Without<ClaimedBy>)>,
) {
    for worker_id in query.iter() {
        println!("Handling work");
        let maybe_nearest_resource = find_nearest_resource(&mut resources);
        if let Some(nearest_resource) = maybe_nearest_resource {
            commands
                .entity(nearest_resource)
                .insert(ClaimedBy(worker_id));
            commands
                .entity(worker_id)
                .insert(LockedOnResource(nearest_resource))
                .insert(MovingToEntity {
                    destination_entity: nearest_resource,
                    sufficient_range: 20.0,
                });
        }
    }
}

fn find_nearest_resource(
    resources: &mut Query<Entity, (With<ResourceChunk>, With<Position>, Without<ClaimedBy>)>,
) -> Option<Entity> {
    resources.iter().next()
}

fn handle_arrivals(
    mut commands: Commands,
    mut arrival_events: EventReader<ArrivedToEntityEvent>,
    resources: Query<&ResourceChunk>,
    mut pickers: Query<(&LockedOnResource, Option<&mut CarryingResources>)>,
) {
    for event in arrival_events.iter() {
        let worker_id = event.moving_entity;
        if let Ok((locked_on_resource, maybe_carrying_resources)) = pickers.get_mut(worker_id) {
            let resource_chunk = resources.get(locked_on_resource.0).unwrap();
            commands.entity(locked_on_resource.0).despawn();
            if let Some(mut carrying_resources) = maybe_carrying_resources {
                commands.entity(worker_id).remove::<LockedOnResource>();
                carrying_resources.0.push(*resource_chunk);
            } else {
                commands
                    .entity(worker_id)
                    .remove::<LockedOnResource>()
                    .insert(CarryingResources(vec![*resource_chunk]));
            }
        }
    }
}

pub fn plan_resource_gathering(commands: &mut Commands, worker_id: Entity) {
    let units_of_work = 70.0;

    commands
        .entity(worker_id)
        .insert(TaskedWithGatheringResources);
}
