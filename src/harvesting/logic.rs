use crate::{
    common::{ClaimedBy, Countdown},
    items::{CarrierInventory, ItemPrefabMap},
    plants::PlantResourceProducer,
    tasks::{CreatureTaskType, IdlingCreature},
};
use bevy::prelude::{Commands, Component, Entity, Query, Res};

#[derive(Component)]
pub struct Harvester {
    target_id: Entity,
}

#[derive(Component)]
pub struct HarvestBatchCountdown(Countdown);

pub fn handle_task_progress(
    mut commands: Commands,
    mut harversters_query: Query<(
        Entity,
        &mut CarrierInventory,
        &Harvester,
        &mut HarvestBatchCountdown,
    )>,
    mut producers: Query<&mut PlantResourceProducer>,
    items: Res<ItemPrefabMap>,
) {
    for (worker_id, mut inventory, tree_cutter, mut harvest_batch_countdown) in
        &mut harversters_query
    {
        if let Ok(mut producer) = producers.get_mut(tree_cutter.target_id) {
            if harvest_batch_countdown.0.tick_yield() {
                produce(&mut producer, &mut inventory, &items);
                println!("Inventory now has {:?}", inventory);
                cleanup(&mut commands, worker_id, Some(tree_cutter.target_id));
            }
        } else {
            cleanup(&mut commands, worker_id, None);
        }
    }
}

pub fn start_harvesting(
    commands: &mut Commands,
    worker_id: Entity,
    target_id: Entity,
    performance: f32,
) {
    commands.entity(target_id).insert(ClaimedBy(worker_id));
    commands.entity(worker_id).insert((
        Harvester { target_id },
        HarvestBatchCountdown(Countdown::new((100.0 / performance).ceil() as u32)), // TODO: make countdown worker performance-related
    ));
}

fn produce(
    resource_producer: &mut PlantResourceProducer,
    receiver_inventory: &mut CarrierInventory,
    items: &Res<ItemPrefabMap>,
) {
    let prefab = items
        .0
        .get(&resource_producer.current.prefab_id)
        .unwrap()
        .clone();

    receiver_inventory.accept(&prefab, &mut resource_producer.current);
}

fn cleanup(commands: &mut Commands, worker_id: Entity, maybe_target_id: Option<Entity>) {
    commands
        .entity(worker_id)
        .remove::<(CreatureTaskType, Harvester, HarvestBatchCountdown)>()
        .insert(IdlingCreature);

    if let Some(target_id) = maybe_target_id {
        commands.entity(target_id).remove::<ClaimedBy>();
    }
}
