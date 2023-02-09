use std::ops::Range;

use bevy::prelude::{Component, Query};
use bevy_turborand::prelude::*;

use crate::{
    common::VariatingCountdown,
    items::{ItemGroup, ItemPrefabId},
};

// For resources that are produced and replenished without the neity being destroyed (a bush producing berries)
#[derive(Component, Clone, Debug)]
pub struct PlantResourceProducer {
    pub current: ItemGroup,
    max_quantity: usize,
    countdown: VariatingCountdown,
}
impl PlantResourceProducer {
    pub fn new(
        item_prefab_id: ItemPrefabId,
        max_quantity: usize,
        period_range: Range<usize>,
        rng: &mut RngComponent,
    ) -> Self {
        PlantResourceProducer {
            current: ItemGroup {
                quantity: 0,
                prefab_id: item_prefab_id,
            },
            max_quantity,
            countdown: VariatingCountdown::new(rng, period_range),
        }
    }

    fn tick(&mut self, rng: &mut RngComponent) {
        self.countdown.tick(rng);
        if self.countdown.is_done() {
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
