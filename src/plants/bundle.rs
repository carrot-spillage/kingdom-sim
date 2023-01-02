use bevy::prelude::{Bundle, Component};

use crate::tree::SimpleDestructible;

#[derive(Component)]
pub struct Plant {
    pub prefab_id: usize,
}

#[derive(Component)]
pub struct Germinating;

#[derive(Component)]
pub struct Growing;

#[derive(Bundle)]
pub struct PlantBundle {
    pub growing: Growing,
    pub germinating: Germinating,
    pub simple_destructible: SimpleDestructible,
}
