use bevy::prelude::{Component, Query};
use rand::Rng;

use crate::items::ItemPrefabId;

use super::bundle::{Growing, Range};

#[derive(Component, Clone, Debug)]
pub struct IntrinsicPlantResourceGrower {
    pub item_prefab_id: ItemPrefabId,
    pub max_quantity: usize,
    pub current_quantity: usize,
}
impl IntrinsicPlantResourceGrower {
    pub fn new(
        item_prefab_id: ItemPrefabId,
        intrinsic_resource_max_quantity_range: Range<usize>,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let max_quantity = rng.gen_range(
            intrinsic_resource_max_quantity_range.from..intrinsic_resource_max_quantity_range.to,
        );
        IntrinsicPlantResourceGrower {
            max_quantity,
            current_quantity: 0,
            item_prefab_id,
        }
    }

    pub fn update(&mut self, maturity: f32) {
        self.current_quantity = (maturity * self.max_quantity as f32).ceil() as usize;
    }
}

pub fn grow_resource(mut growers: Query<(&Growing, &mut IntrinsicPlantResourceGrower)>) {
    for (growing, mut grower) in &mut growers {
        grower.update(growing.maturity);
    }
}
