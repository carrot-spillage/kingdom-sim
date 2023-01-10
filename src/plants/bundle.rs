use bevy::prelude::{Bundle, Component, Vec2};
use rand::Rng;

use crate::{tree::SimpleDestructible, common::Countdown};

#[derive(Component)]
pub struct Plant {
    pub prefab_id: usize,
}

#[derive(Component)]
pub struct MaturePlant;

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Clone, Copy, Debug)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
pub struct Range<T> {
    pub from: T,
    pub to: T
}

#[derive(Component, serde::Deserialize, bevy::reflect::TypeUuid, Clone, Copy, Debug)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c4b"]
pub struct Germinator {
    pub radius: usize,
    pub period_range: Range<usize>,
}

impl Germinator {
    pub fn gen_countdown(&self) -> GerminatorCountdown {
        let mut rng = rand::thread_rng();
        let rand_period = rng.gen_range(self.period_range.from..self.period_range.to);
        GerminatorCountdown(Countdown::new(rand_period))
    }

    pub fn gen_offset(&self) -> Vec2 {
        let mut rng = rand::thread_rng();
        let rand_distance_x = rng.gen_range(0..self.radius);
        let rand_distance_y = rng.gen_range(0..self.radius);

        Vec2::new(rand_distance_x as f32, rand_distance_y as f32)
    }
}

#[derive(Component, Clone, Copy, Debug)]

pub struct GerminatorCountdown(pub Countdown);

#[derive(Component, Clone, Copy, Debug)]
pub struct Growing {
    pub rate: f32,
    pub maturity: f32,
}

#[derive(Component, Clone, Debug)]
pub struct PlantName(pub String);

#[derive(Bundle, Clone, Debug)]
pub struct PlantBundle {
    pub growing: Growing,
    pub countdown: GerminatorCountdown,
    pub germinating: Germinator,
    pub simple_destructible: SimpleDestructible,
    pub name: PlantName
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Debug)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c4a"]
pub struct PlantPrefab {
    pub name: String,
    pub texture_path: String,
    pub health: usize,
    pub growth_rate: f32,
    pub germinating: Germinator,
}

impl PlantPrefab {
    pub fn to_plant_bundle(&self) -> PlantBundle {
        PlantBundle {
            name: PlantName(self.name.clone()),
            countdown: self.germinating.gen_countdown(),
            germinating: self.germinating,
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