pub mod overlay_tilemap;
mod soil_fertility;
mod tile_image;

use bevy::prelude::Component;

pub use soil_fertility::{SoilFertility, SoilFertilityLayerPlugin};
pub use tile_image::generate_tile_image;
#[derive(Component)]
pub struct Humidity(pub f32); // 0..1

pub struct Wind {
    pub speed: f32,
    pub direction: f32,
}
