use bevy::prelude::{Added, Commands, Entity, Query, Res};

use crate::{
    common::NeedsDestroying,
    items::{spawn_item_group, ItemPrefabMap},
    movement::Position,
};

use super::{IntrinsicPlantResourceGrower, PlantResourceProducer};

pub fn break_into_resources(
    mut commands: Commands,
    items: Res<ItemPrefabMap>,
    to_be_destroyed: Query<
        (
            Entity,
            &Position,
            &IntrinsicPlantResourceGrower,
            &PlantResourceProducer,
        ),
        Added<NeedsDestroying>,
    >,
) {
    for (entity, position, grower, producer) in &to_be_destroyed {
        {
            let item_group = grower.item_group;
            let (prefab, texture) = items.0.get(&item_group.prefab_id).unwrap();
            spawn_item_group(
                &mut commands,
                texture.clone(),
                item_group,
                position.0,
                false,
            );
        }
        {
            let item_group = producer.current;
            let (prefab, texture) = items.0.get(&item_group.prefab_id).unwrap();

            spawn_item_group(
                &mut commands,
                texture.clone(),
                item_group,
                position.0,
                false,
            );
        }

        commands.entity(entity).despawn();
    }
}
