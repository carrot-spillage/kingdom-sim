use std::f32::consts::PI;

use bevy::prelude::{Bundle, Component, Vec2};
use rand::{Rng, rngs::ThreadRng};

use crate::{common::Countdown, tree::SimpleDestructible};

#[derive(Component, Clone, Debug, Hash, PartialEq, Eq)]
pub struct PlantPrefabId(pub usize);

#[derive(Component, Clone, Debug)]
pub struct PlantName(pub &'static str);

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Clone, Copy, Debug)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
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
    pub growing: Growing,
    pub germinator_params: GerminatorParams,
    pub simple_destructible: SimpleDestructible,
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Debug)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c4a"]
pub struct PlantPrefab {
    pub name: String,
    pub texture_path: String,
    pub health: usize,
    pub growth_rate: f32,
    pub germinator_params: GerminatorParams,
}

impl PlantPrefab {
    pub fn to_plant_bundle(&self, id: usize) -> PlantBundle {
        PlantBundle {
            prefab_id: PlantPrefabId(id),
            germinator_params: self.germinator_params,
            growing: Growing {
                maturity: 0.0,
                rate: self.growth_rate,
            },
            simple_destructible: SimpleDestructible {
                max_health: self.health as f32,
                current_health: self.health as f32,
            },
        }
    }
}
