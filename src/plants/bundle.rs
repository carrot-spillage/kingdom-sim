use std::ops::Range;

use bevy::prelude::{Bundle, Component};

use crate::tree::SimpleDestructible;

#[derive(Component)]
pub struct Plant {
    pub prefab_id: usize,
}

#[derive(Component, serde::Deserialize, bevy::reflect::TypeUuid, Clone, Copy, Debug)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c4b"]
pub struct Germinating {
    pub radius: usize,
    pub period: usize,
    // pub quantity_range: Range<usize>,
}

#[derive(Component, Clone, Copy)]
pub struct Growing {
    growth_speed: f32
}

#[derive(Bundle, Clone, Copy)]
pub struct PlantBundle {
    pub growing: Growing,
    pub germinating: Germinating,
    pub simple_destructible: SimpleDestructible,
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Debug)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c4a"]
pub struct PlantPrefab {
    pub name: String,
    pub health: usize,
    pub growth_speed: f32,
    pub germinating: Germinating,
}