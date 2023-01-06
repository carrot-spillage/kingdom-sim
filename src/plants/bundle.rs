use std::ops::Range;

use bevy::prelude::{Bundle, Component};

use crate::tree::SimpleDestructible;

#[derive(Component)]
pub struct Plant {
    pub prefab_id: usize,
}

#[derive(Component, serde::Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c4b"]
pub struct Germinating {
    pub radius: usize,
    pub period: usize,
    pub quantity_range: Range<usize>
}

#[derive(Component)]
pub struct Growing;

#[derive(Bundle)]
pub struct PlantBundle {
    pub growing: Growing,
    pub germinating: Germinating,
    pub simple_destructible: SimpleDestructible,
}


#[derive(serde::Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c4a"]
pub(crate) struct PlantPrefab {
    pub name: String,
    pub growth_speed: f32,
    pub germinating: Germinating,
    pub health: usize,
}
