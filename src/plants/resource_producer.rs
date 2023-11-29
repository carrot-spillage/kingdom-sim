use std::ops::Range;

use bevy::{
    ecs::event::EventReader,
    prelude::{Component, Query},
};

use crate::{
    items::{ItemBatch, ItemPrefabId},
    timer_plugin::{ElapsedEvent, Timed, TimerSettings},
};

// For resources that are produced and replenished without the neity being destroyed (a bush producing berries)
#[derive(Component, Clone, Debug)]
pub struct PlantResourceProducer {
    pub current: ItemBatch,
    pub max_quantity: u32,
    timer_settings: TimerSettings,
}
impl PlantResourceProducer {
    pub fn new(item_prefab_id: ItemPrefabId, max_quantity: u32, period_range: Range<u32>) -> Self {
        PlantResourceProducer {
            current: ItemBatch {
                quantity: 0,
                prefab_id: item_prefab_id,
            },
            max_quantity,
            timer_settings: TimerSettings::RepeatedRandom(period_range.start, period_range.end),
        }
    }
}

impl Timed for PlantResourceProducer {
    fn get_timer_settings(&self) -> TimerSettings {
        self.timer_settings.clone()
    }
}

pub fn produce_resources(
    mut producers: Query<&mut PlantResourceProducer>,
    mut elapsed_producers: EventReader<ElapsedEvent<PlantResourceProducer>>,
) {
    for event in &mut elapsed_producers {
        let mut producer = producers.get_mut(event.entity).unwrap();
        if producer.current.quantity < producer.max_quantity {
            producer.current.quantity += 1;
        }
    }
}
