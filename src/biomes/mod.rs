mod soil_fertility;

use bevy::prelude::{
    App, Commands, Component, IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnUpdate, Plugin,
    Query,
};

pub use soil_fertility::{SoilFertility, SoilFertilityLayerPlugin};

#[derive(Component)]
pub struct Humidity(pub f32); // 0..1

pub struct Wind {
    pub speed: f32,
    pub direction: f32,
}
