mod soil_fertility;

use bevy::prelude::Component;

pub use soil_fertility::{SoilFertility, SoilFertilityLayerPlugin};

#[derive(Component)]
pub struct Humidity(pub f32); // 0..1

pub struct Wind {
    pub speed: f32,
    pub direction: f32,
}
