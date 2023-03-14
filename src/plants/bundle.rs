use crate::{
    common::{SimpleDestructible, VariableCountdown},
    items::ItemPrefabId,
};
use bevy::prelude::{Bundle, Component, Handle, Image, ResMut, Vec2};
use bevy_turborand::prelude::*;
use std::f32::consts::PI;

use super::{
    intrinsic_resource::IntrinsicPlantResourceGrower, resource_producer::PlantResourceProducer,
    PlantMaturityStage,
};

#[derive(
    Component, serde::Deserialize, bevy::reflect::TypeUuid, Clone, Copy, Debug, Hash, PartialEq, Eq,
)]
#[uuid = "407e6caf-2901-437a-b2e6-5ca256de6b2a"]
pub struct PlantPrefabId(pub u32);

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
    pub radius: u32,
    pub period_range: Range<u32>,
}

#[derive(Component, Clone, Debug)]
pub struct Germinator {
    countdown: VariableCountdown,
    params: GerminatorParams,
}

impl Germinator {
    pub fn new(params: GerminatorParams) -> Self {
        Germinator {
            countdown: VariableCountdown::new(params.period_range.from..params.period_range.to),
            params,
        }
    }

    pub fn try_produce(&mut self, rng: &mut RngComponent) -> Option<Vec2> {
        if self.countdown.tick_yield(rng) {
            let rand_offset_x = rng.f32_normalized() * self.params.radius as f32;
            let rand_offset_y = (rng.f32_normalized() * PI).sin() * self.params.radius as f32;

            return Some(Vec2::new(rand_offset_x as f32, rand_offset_y as f32));
        }

        None
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
    pub rng: RngComponent,
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Debug, Clone, Copy)]
#[uuid = "fd8aa8ff-bb48-4572-a6d8-7e7dc1fec9a7"]
pub struct IntrinsicResourceParams {
    pub max_quantity_range: Range<u32>,
    pub item_prefab_id: ItemPrefabId,
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Debug, Clone, Copy)]
#[uuid = "f36d1e36-3e4f-4608-b9f7-5bc1b9f61053"]
pub struct ResourceProducerParams {
    pub max_quantity: u32,
    pub period_range: Range<u32>,
    pub item_prefab_id: ItemPrefabId,
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Debug, Clone, Copy)]
#[uuid = "f36d1e36-3e4f-4608-b9f7-5bc1b9f62055"]
pub struct Size {
    pub x: f32,
    pub y: f32,
}

impl Size {
    pub fn to_vec(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Debug, Clone)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c4a"]
pub struct PlantPrefab<T = Handle<Image>, V = Vec2> {
    pub id: PlantPrefabId,
    pub name: String,
    pub textures: PlantPrefabTextureSet<T>,
    pub collision_box: V,
    pub health: u32,
    pub growth_rate: f32,
    pub germinator: GerminatorParams,
    pub intrinsic_resource: Option<IntrinsicResourceParams>,
    pub resource_producer: Option<ResourceProducerParams>,
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Debug, Clone)]
#[uuid = "413be529-bfeb-41b3-9db0-4b44380a2c4a"]
pub struct PlantPrefabTextureSet<T> {
    pub default: T,
}

impl<T> PlantPrefab<T> {
    pub fn to_plant_components(
        &self,
        maturity_state: &PlantMaturityStage,
        global_rng: &mut ResMut<GlobalRng>,
    ) -> (
        PlantBundle,
        Option<IntrinsicPlantResourceGrower>,
        Option<PlantResourceProducer>,
        Option<Growing>,
        Option<Germinator>,
    ) {
        let mut rng = RngComponent::from(global_rng);
        let maybe_grower = self.intrinsic_resource.map(|x| {
            IntrinsicPlantResourceGrower::new(
                x.item_prefab_id,
                x.max_quantity_range.from..x.max_quantity_range.to,
                &mut rng,
            )
        });
        (
            PlantBundle {
                prefab_id: self.id,
                germinator_params: self.germinator,
                simple_destructible: SimpleDestructible {
                    max_health: self.health as f32,
                    current_health: self.health as f32,
                },
                rng,
            },
            maybe_grower,
            self.resource_producer.map(|x| {
                PlantResourceProducer::new(
                    x.item_prefab_id,
                    x.max_quantity,
                    x.period_range.from..x.period_range.to,
                )
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
