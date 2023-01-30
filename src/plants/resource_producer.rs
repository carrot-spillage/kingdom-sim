use bevy::prelude::{Component, Query};
use rand::Rng;

use super::bundle::Range;
use crate::{
    common::Countdown,
    items::{ItemGroup, ItemPrefabId},
};

// For resources that are produced and replenished without the neity being destroyed (a bush producing berries)
#[derive(Component, Clone, Debug)]
pub struct PlantResourceProducer {
    pub current: ItemGroup,
    max_quantity: usize,
    period_range: Range<usize>,
    countdown: Countdown,
}
impl PlantResourceProducer {
    pub fn new(
        item_prefab_id: ItemPrefabId,
        max_quantity: usize,
        period_range: Range<usize>,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let rand_period = rng.gen_range(period_range.from..period_range.to);

        PlantResourceProducer {
            current: ItemGroup {
                quantity: 0,
                prefab_id: item_prefab_id,
            },
            period_range,
            max_quantity,
            countdown: Countdown::new(rand_period),
        }
    }

    fn tick(&mut self) {
        self.countdown.tick();
        if self.countdown.is_done() {
            if self.current.quantity < self.max_quantity {
                self.current.quantity += 1;
            }
            let mut rng = rand::thread_rng();
            let rand_period = rng.gen_range(self.period_range.from..self.period_range.to);
            self.countdown = Countdown::new(rand_period);
        }
    }

    pub fn max_out(&mut self) {
        // TODO: this looks like a hack. maybe it asks for redesigning the whole producer/countdown
        self.current.quantity = self.max_quantity
    }
}

pub fn produce_resources(mut producer_lists: Query<&mut PlantResourceProducer>) {
    for mut producer in &mut producer_lists {
        producer.tick();
    }
}
