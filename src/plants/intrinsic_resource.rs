use bevy::prelude::{Component, Query};
use rand::Rng;

use crate::items::{ItemGroup, ItemPrefabId};

use super::bundle::{Growing, Range};

#[derive(Component, Clone, Debug)]
pub struct IntrinsicPlantResourceGrower {
    pub item_group: ItemGroup,
    pub max_quantity: usize,
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
            item_group: ItemGroup {
                prefab_id: item_prefab_id,
                quantity: 0,
            },
        }
    }

    pub fn update(&mut self, maturity: f32) {
        self.item_group.quantity = (maturity * self.max_quantity as f32).ceil() as usize;
    }

    // TODO: this looks like a hack. maybe it asks for redesigning the whole struct/countdown
    pub(crate) fn max_out(&mut self) {
        self.item_group.quantity = self.max_quantity;
    }
}

pub fn grow_resource(mut growers: Query<(&Growing, &mut IntrinsicPlantResourceGrower)>) {
    for (growing, mut grower) in &mut growers {
        grower.update(growing.maturity);
    }
}
