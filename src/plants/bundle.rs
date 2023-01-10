use bevy::prelude::{Bundle, Component};

use crate::tree::SimpleDestructible;

#[derive(Component)]
pub struct Plant {
    pub prefab_id: usize,
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Clone, Copy, Debug)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
pub struct Range<T> {
    pub from: T,
    pub to: T
}

#[derive(Component, serde::Deserialize, bevy::reflect::TypeUuid, Clone, Copy, Debug)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c4b"]
pub struct Germinating {
    pub radius: usize,
    pub period_range: Range<usize>,
    pub quantity_range: Range<usize>,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Growing {
    pub rate: f32,
    pub maturity: f32,
}

#[derive(Bundle, Clone, Copy, Debug)]
pub struct PlantBundle {
    pub growing: Growing,
    pub germinating: Germinating,
    pub simple_destructible: SimpleDestructible,
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Debug)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c4a"]
pub struct PlantPrefab {
    pub name: String,
    pub texture_path: String,
    pub health: usize,
    pub growth_rate: f32,
    pub germinating: Germinating,
}

impl PlantPrefab {
    pub fn to_plant_bundle(&self) -> PlantBundle {
        PlantBundle {
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