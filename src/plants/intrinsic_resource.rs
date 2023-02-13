use std::ops::Range;

use bevy::prelude::{Component, Query};
use bevy_turborand::prelude::*;

use crate::items::{ItemBatch, ItemPrefabId};

use super::bundle::Growing;

#[derive(Component, Clone, Debug)]
pub struct IntrinsicPlantResourceGrower {
    pub item_batch: ItemBatch,
    pub max_quantity: u32,
}
impl IntrinsicPlantResourceGrower {
    pub fn new(
        item_prefab_id: ItemPrefabId,
        intrinsic_resource_max_quantity_range: Range<u32>,
        rng: &mut RngComponent,
    ) -> Self {
        let max_quantity = rng.u32(intrinsic_resource_max_quantity_range);
        IntrinsicPlantResourceGrower {
            max_quantity,
            item_batch: ItemBatch {
                prefab_id: item_prefab_id,
                quantity: 0,
            },
        }
    }

    pub fn update(&mut self, maturity: f32) {
        self.item_batch.quantity = (maturity * self.max_quantity as f32).ceil() as u32;
    }

    // TODO: this looks like a hack. maybe it asks for redesigning the whole struct/countdown
    pub(crate) fn max_out(&mut self) {
        self.item_batch.quantity = self.max_quantity;
    }
}

pub fn grow_resource(mut growers: Query<(&Growing, &mut IntrinsicPlantResourceGrower)>) {
    for (growing, mut grower) in &mut growers {
        grower.update(growing.maturity);
    }
}
