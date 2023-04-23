use bevy::prelude::{App, Component, IntoSystemConfig, OnUpdate, Plugin, Res};
use chrono::Timelike;

use crate::{datetime::GameTime, GameState};

#[derive(Component)]
pub struct Humidity(pub f32); // 0..1

#[derive(Component)]
pub struct SoilFertility(pub f32); // 0..1

#[derive(Component)]
pub struct Temperature(pub f32); // -50..+50

pub struct Wind {
    pub speed: f32,
    pub direction: f32,
}

pub struct TemperaturePlugin;

impl Plugin for TemperaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_temperature.in_set(OnUpdate(GameState::Playing)));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn update_temperature(game_time: Res<GameTime>) {
    if game_time.0.hour() < 6 {}
}
