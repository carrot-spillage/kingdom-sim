use bevy::prelude::{
    App, Commands, Component, IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnUpdate, Plugin,
    Query,
};

use crate::GameState;

use super::SunAltitude;

#[derive(Component)]
pub struct Temperature(pub f32); // -50..+50

#[derive(Component)]
pub struct BaseTemperature(pub f32); // -50..+50

pub struct TemperaturePlugin;

impl Plugin for TemperaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(init_temperature.in_schedule(OnEnter(GameState::Playing)))
            .add_system(update_temperature.in_set(OnUpdate(GameState::Playing)));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn init_temperature(mut commands: Commands) {
    commands.spawn_empty().insert(BaseTemperature(16.0));
}

fn update_temperature(
    sun_altitude_q: Query<&SunAltitude>,
    base_temperature_q: Query<&BaseTemperature>,
    mut temperature_q: Query<&mut Temperature>,
) {
    let sun_altitude = sun_altitude_q.single().0;
    let base_temperature = base_temperature_q.single().0;
    temperature_q.single_mut().0 = calc_temperature(sun_altitude, base_temperature);
}

fn calc_temperature(sun_altitude: f32, base_temperature: f32) -> f32 {
    let factor = if sun_altitude > 0.0 { 1.0 } else { 0.3 };
    let range = 8.0;
    base_temperature + factor * range * sun_altitude
}
