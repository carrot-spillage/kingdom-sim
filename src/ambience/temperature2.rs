use bevy::prelude::{
    in_state, App, Commands, Component, IntoSystemConfigs, OnEnter, Plugin, Query, Update,
};

use crate::GameState;

use super::SunAltitude;

pub struct Temperature2Plugin;

impl Plugin for Temperature2Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), init_temperature)
            .add_systems(
                Update,
                update_temperature.run_if(in_state(GameState::Playing)),
            );
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
