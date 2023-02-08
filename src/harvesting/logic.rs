use crate::{
    common::{ClaimedBy, Countdown},
    items::{CarrierInventory, ItemGroup, ItemPrefabMap, ItemTakingResult},
    plants::PlantResourceProducer,
    tasks::IdlingWorker,
};
use bevy::prelude::{Commands, Component, Entity, Query, Res};

enum AdvanceResult {
    Continuing(Countdown),
    Completed,
}

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
            harvest_batch_countdown.0.tick();
            if harvest_batch_countdown.0.is_done() {
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
        HarvestBatchCountdown(Countdown::new((100.0 / performance).ceil() as usize)), // TODO: make countdown worker performance-related
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
        .0
        .clone();

    let rest = receiver_inventory.put_and_get_rest(&prefab, resource_producer.current);

    resource_producer.current = rest.unwrap_or(ItemGroup {
        quantity: 0,
        prefab_id: resource_producer.current.prefab_id,
    });
}

fn cleanup(commands: &mut Commands, worker_id: Entity, maybe_target_id: Option<Entity>) {
    commands
        .entity(worker_id)
        .remove::<(Harvester, HarvestBatchCountdown)>()
        .insert(IdlingWorker);

    if let Some(target_id) = maybe_target_id {
        commands.entity(target_id).remove::<ClaimedBy>();
    }
}
