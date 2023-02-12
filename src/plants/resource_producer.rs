use std::ops::Range;

use bevy::prelude::{Component, Query};
use bevy_turborand::prelude::*;

use crate::{
    common::VariableCountdown,
    items::{ItemGroup, ItemPrefabId},
};

// For resources that are produced and replenished without the neity being destroyed (a bush producing berries)
#[derive(Component, Clone, Debug)]
pub struct PlantResourceProducer {
    pub current: ItemGroup,
    max_quantity: u32,
    countdown: VariableCountdown,
}
impl PlantResourceProducer {
    pub fn new(item_prefab_id: ItemPrefabId, max_quantity: u32, period_range: Range<u32>) -> Self {
        PlantResourceProducer {
            current: ItemGroup {
                quantity: 0,
                prefab_id: item_prefab_id,
            },
            max_quantity,
            countdown: VariableCountdown::new(period_range),
        }
    }

    fn tick(&mut self, rng: &mut RngComponent) {
        if self.countdown.tick_yield(rng) {
            if self.current.quantity < self.max_quantity {
                self.current.quantity += 1;
            }
        }
    }

    pub fn max_out(&mut self) {
        // TODO: this looks like a hack. maybe it asks for redesigning the whole producer/countdown
        self.current.quantity = self.max_quantity
    }
}

pub fn produce_resources(
    mut producer_lists: Query<(&mut PlantResourceProducer, &mut RngComponent)>,
) {
    for (mut producer, mut rng) in &mut producer_lists {
        producer.tick(&mut rng);
    }
}
