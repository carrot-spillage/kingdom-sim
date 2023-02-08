use crate::{common::Countdown, common::SimpleDestructible, items::ItemPrefabId};
use bevy::prelude::{Bundle, Component, Vec2};
use bevy_turborand::prelude::*;
use rand::{rngs::ThreadRng, Rng};
use std::f32::consts::PI;

use super::{
    intrinsic_resource::IntrinsicPlantResourceGrower, resource_producer::PlantResourceProducer,
    PlantMaturityStage,
};

#[derive(
    Component, serde::Deserialize, bevy::reflect::TypeUuid, Clone, Copy, Debug, Hash, PartialEq, Eq,
)]
#[uuid = "407e6caf-2901-437a-b2e6-5ca256de6b2a"]
pub struct PlantPrefabId(pub usize);

#[derive(Component, Clone, Debug)]
pub struct PlantName(pub &'static str);

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Clone, Copy, Debug)]
#[uuid = "c1b29b63-2032-413c-bb10-bb0e9b54f7b2"]
pub struct Range<T> {
    pub from: T,
    pub to: T,
}

#[derive(Component, serde::Deserialize, bevy::reflect::TypeUuid, Clone, Copy, Debug)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c4b"]
pub struct GerminatorParams {
    pub radius: usize,
    pub period_range: Range<usize>,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Germinator {
    countdown: Countdown,
    params: GerminatorParams,
}

impl Germinator {
    pub fn new(params: GerminatorParams) -> Self {
        let mut rng = rand::thread_rng();

        Germinator {
            countdown: Self::gen_countdown(&mut rng, &params.period_range),
            params,
        }
    }

    pub fn try_produce(&mut self) -> Option<Vec2> {
        self.countdown.tick();
        if self.countdown.is_done() {
            let mut rng = rand::thread_rng();

            self.countdown = Self::gen_countdown(&mut rng, &self.params.period_range);

            let radius_range = -(self.params.radius as f32)..self.params.radius as f32;
            let rand_offset_x = rng.gen_range(radius_range);
            let rand_offset_y = rng.gen_range(-PI..PI).sin() * self.params.radius as f32;

            return Some(Vec2::new(rand_offset_x as f32, rand_offset_y as f32));
        }

        None
    }

    fn gen_countdown(rng: &mut ThreadRng, period_range: &Range<usize>) -> Countdown {
        let rand_period = rng.gen_range(period_range.from..period_range.to);
        Countdown::new(rand_period)
    }
}

// let germ_position = position.0 + germinator_params.gen_offset().extend(0.0);
// let (bundle, texture) = plant_bundle_map.0.get(&plant_name.0.clone()).unwrap();
// plant_germ(&mut commands, bundle.clone(), texture.clone(), germ_position);

#[derive(Component, Clone, Copy, Debug)]
pub struct Growing {
    pub rate: f32,
    pub maturity: f32,
}

#[derive(Bundle, Clone, Debug)]
pub struct PlantBundle {
    pub prefab_id: PlantPrefabId,
    pub germinator_params: GerminatorParams,
    pub simple_destructible: SimpleDestructible,
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Debug, Clone, Copy)]
#[uuid = "fd8aa8ff-bb48-4572-a6d8-7e7dc1fec9a7"]
pub struct IntrinsicResourceParams {
    pub max_quantity_range: Range<usize>,
    pub item_prefab_id: ItemPrefabId,
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Debug, Clone, Copy)]
#[uuid = "f36d1e36-3e4f-4608-b9f7-5bc1b9f61053"]
pub struct ResourceProducerParams {
    pub max_quantity: usize,
    pub period_range: Range<usize>,
    pub item_prefab_id: ItemPrefabId,
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Debug, Clone)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c4a"]
pub struct PlantPrefab {
    pub id: PlantPrefabId,
    pub name: String,
    pub texture: String,
    pub health: usize,
    pub growth_rate: f32,
    pub germinator: GerminatorParams,
    pub intrinsic_resource: Option<IntrinsicResourceParams>,
    pub resource_producer: Option<ResourceProducerParams>,
}

impl PlantPrefab {
    pub fn to_plant_components(
        &self,
        maturity_state: &PlantMaturityStage,
    ) -> (
        PlantBundle,
        Option<IntrinsicPlantResourceGrower>,
        Option<PlantResourceProducer>,
        Option<Growing>,
        Option<Germinator>,
    ) {
        (
            PlantBundle {
                prefab_id: self.id,
                germinator_params: self.germinator,
                simple_destructible: SimpleDestructible {
                    max_health: self.health as f32,
                    current_health: self.health as f32,
                },
            },
            self.intrinsic_resource
                .map(|x| IntrinsicPlantResourceGrower::new(x.item_prefab_id, x.max_quantity_range)),
            self.resource_producer.map(|x| {
                PlantResourceProducer::new(x.item_prefab_id, x.max_quantity, x.period_range)
            }),
            match maturity_state {
                PlantMaturityStage::Germ => Some(Growing {
                    maturity: 0.0,
                    rate: self.growth_rate,
                }),
                PlantMaturityStage::FullyGrown => None,
            },
            match maturity_state {
                PlantMaturityStage::Germ => None,
                PlantMaturityStage::FullyGrown => Some(Germinator::new(self.germinator)),
            },
        )
    }
}
