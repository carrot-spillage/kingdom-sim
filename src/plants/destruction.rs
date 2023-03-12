use bevy::prelude::{Added, Commands, Entity, Query, Res};

use crate::{
    common::NeedsDestroying,
    create_world::WorldParams,
    items::{spawn_item_batch, ItemPrefabMap},
    movement::Position,
};

use super::{IntrinsicPlantResourceGrower, PlantResourceProducer};

pub fn break_into_resources(
    mut commands: Commands,
    items: Res<ItemPrefabMap>,
    world_params: Res<WorldParams>,
    to_be_destroyed: Query<
        (
            Entity,
            &Position,
            Option<&IntrinsicPlantResourceGrower>,
            Option<&PlantResourceProducer>,
        ),
        Added<NeedsDestroying>,
    >,
) {
    for (entity, position, maybe_grower, maybe_producer) in &to_be_destroyed {
        if let Some(grower) = maybe_grower {
            let item_batch = grower.item_batch;
            if item_batch.quantity == 0 {
                continue;
            }
            let (prefab, texture) = items.0.get(&item_batch.prefab_id).unwrap();

            spawn_item_batch(
                &mut commands,
                texture.clone(),
                item_batch,
                position.0,
                &world_params,
            );
        }
        if let Some(producer) = maybe_producer {
            let item_batch = producer.current;
            if item_batch.quantity == 0 {
                continue;
            }
            let (prefab, texture) = items.0.get(&item_batch.prefab_id).unwrap();

            spawn_item_batch(
                &mut commands,
                texture.clone(),
                item_batch,
                position.0,
                &world_params,
            );
        }

        commands.entity(entity).despawn();
    }
}
