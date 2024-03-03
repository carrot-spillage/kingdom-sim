use crate::{
    common::SimpleDestructible,
    items::ItemPrefabId,
    timer_plugin::{Timed, TimerSettings},
};
use bevy::{
    prelude::{Bundle, Component, Handle, Image, ResMut, Vec2},
    reflect::TypePath,
};
use bevy_turborand::prelude::*;

use super::{
    intrinsic_resource::IntrinsicPlantResourceGrower, resource_producer::PlantResourceProducer,
    PlantMaturityStage,
};

#[derive(Component, serde::Deserialize, TypePath, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct PlantPrefabId(pub u32);

#[derive(Component, Clone, Debug)]
pub struct PlantName(pub &'static str);

#[derive(serde::Deserialize, TypePath, Clone, Copy, Debug)]
pub struct Range<T> {
    pub from: T,
    pub to: T,
}

#[derive(Component, serde::Deserialize, TypePath, Clone, Copy, Debug)]
pub struct GerminatorParams {
    pub radius: u32,
    pub period_range: Range<u32>,
}

#[derive(Component, Clone, Debug)]
pub struct Germinator {
    pub timer_settings: TimerSettings,
}

impl Timed for Germinator {
    fn get_timer_settings(&self) -> TimerSettings {
        self.timer_settings
    }
}

impl Germinator {
    pub fn new(params: GerminatorParams) -> Self {
        Germinator {
            timer_settings: TimerSettings::RepeatedRandom(
                params.period_range.from,
                params.period_range.to,
            ),
        }
    }
}

// let germ_position = position.0 + germinator_params.gen_offset().extend(0.0);
// let (bundle, texture) = plant_bundle_map.0.get(&plant_name.0.clone()).unwrap();
// plant_germ(&mut commands, bundle.clone(), texture.clone(), germ_position);

#[derive(Component, Clone, Copy, Debug)]
pub struct Growing {
    pub rate: f32,
    pub maturity: f32,
    pub timer_settings: TimerSettings,
}

impl Timed for Growing {
    fn get_timer_settings(&self) -> TimerSettings {
        self.timer_settings
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct PlantBundle {
    pub prefab_id: PlantPrefabId,
    pub germinator_params: GerminatorParams,
    pub simple_destructible: SimpleDestructible,
    pub rng: RngComponent,
}

#[derive(serde::Deserialize, TypePath, Debug, Clone, Copy)]
pub struct IntrinsicResourceParams {
    pub max_quantity_range: Range<u32>,
    pub item_prefab_id: ItemPrefabId,
}

#[derive(serde::Deserialize, TypePath, Debug, Clone, Copy)]
pub struct ResourceProducerParams {
    pub max_quantity: u32,
    pub period_range: Range<u32>,
    pub item_prefab_id: ItemPrefabId,
}

#[derive(serde::Deserialize, TypePath, Debug, Clone, Copy)]
pub struct Size {
    pub x: f32,
    pub y: f32,
}

impl Size {
    pub fn to_vec(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
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

#[derive(serde::Deserialize, Debug, Clone)]
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
                    timer_settings: TimerSettings::RepeatedExact(20),
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
