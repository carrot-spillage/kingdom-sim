use std::f32::consts::PI;

use bevy::{
    prelude::{
        in_state, App, Changed, Commands, Component, IntoSystemConfigs, OnEnter, Plugin, Query,
        Res, ResMut, Resource, Update, With,
    },
    utils::HashMap,
};
use bevy_turborand::{DelegatedRng, GlobalRng, RngComponent};
use chrono::{Datelike, NaiveDate, Timelike};

use crate::{datetime::GameTime, GameState};

use super::SunAltitude;

#[derive(Component)]
pub struct Temperature(pub f32); // -50..+50

#[derive(Component)]
pub struct RainIntensity(pub f32); // -50..+50

#[derive(Component)]
pub struct Hour(pub u32); // -50..+50

#[derive(Component)]
pub struct BaseTemperature(pub f32); // -50..+50

#[derive(Resource)]
struct WeatherForYear {
    year: i32,
    daily_temperature: Vec<f32>,
    hourly_rain_intensity: HashMap<DayHour, RainIntensity>,
}

pub struct TemperaturePlugin;

impl Plugin for TemperaturePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WeatherForYear {
            year: 0,
            daily_temperature: vec![],
            hourly_rain_intensity: HashMap::<DayHour, RainIntensity>::new(),
        })
        .add_systems(OnEnter(GameState::Playing), init)
        .add_systems(
            Update,
            (update_hour, update_temperature).run_if(in_state(GameState::Playing)),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn update_hour(game_time: Res<GameTime>, mut hour_q: Query<&mut Hour>) {
    let time = game_time.0.time();
    let hour: u32 = time.hour();
    let mut _hour = hour_q.single_mut();
    if _hour.0 != hour {
        _hour.0 = hour
    }
}

fn init(mut commands: Commands, mut global_rng: ResMut<GlobalRng>) {
    commands.spawn_empty().insert((
        BaseTemperature(16.0),
        Temperature(16.0),
        Hour(0),
        RainIntensity(0.0),
        RngComponent::from(&mut global_rng),
    ));
}

fn update_temperature(
    game_time: Res<GameTime>,
    hour_altitude_q: Query<&SunAltitude, Changed<SunAltitude>>,
    mut daily_temperature_for_year: ResMut<WeatherForYear>,
    base_temperature_q: Query<&BaseTemperature>,
    mut weather_q: Query<(&mut Temperature, &mut RainIntensity)>,
    mut rng_q: Query<&mut RngComponent, With<Temperature>>,
) {
    if let Ok(SunAltitude(altitude)) = hour_altitude_q.get_single() {
        let time = game_time.0.time();
        let naive_date = game_time.0.date_naive();
        let year = naive_date.year();

        if daily_temperature_for_year.year != year {
            let base_temperature = base_temperature_q.single().0;
            let last_day = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();
            let toal_days = last_day.ordinal();
            daily_temperature_for_year.year = year;
            daily_temperature_for_year.daily_temperature =
                generate_daily_temperature_for_year(toal_days, base_temperature);

            let mut rng = rng_q.single_mut();
            daily_temperature_for_year.hourly_rain_intensity =
                generate_hourly_rain_for_year(toal_days, &mut rng);
        }

        let day_base_temperature =
            daily_temperature_for_year.daily_temperature[naive_date.ordinal0() as usize];
        let daily_temperature_change_range = (-0.25, 0.5);

        let temperature = generate_temperature(
            *altitude,
            day_base_temperature,
            daily_temperature_change_range,
        );

        let (mut w_temperature, mut w_rain_itensity) = weather_q.single_mut();
        w_temperature.0 = temperature;
        w_rain_itensity.0 = daily_temperature_for_year
            .hourly_rain_intensity
            .get(&DayHour {
                day: naive_date.day(),
                hour: time.hour(),
            })
            .unwrap()
            .0
    }
}

fn generate_daily_temperature_for_year(total_days: u32, base_temperature: f32) -> Vec<f32> {
    let summer_peak_offset = (total_days as f32 * 0.3) as usize;

    let amplitude = 1.0; // Adjust the amplitude of the sinusoid as needed
    let frequency = 2.0 * PI / (total_days as f32); // Frequency for a yearly cycle (in radians)

    let positive_amplification = 1.8;

    (1..total_days)
        .map(|day| {
            let angle = frequency * (day as f32 - summer_peak_offset as f32);
            let mut value = amplitude * f32::sin(angle) * base_temperature;
            if value > 0.0 {
                value *= positive_amplification;
            }
            value
        })
        .collect()
}

fn generate_temperature(
    altitude: f32,
    base_temperature: f32,
    daily_temperature_change_range: (f32, f32),
) -> f32 {
    let temperature_shift = altitude
        * if altitude > 0.0 {
            daily_temperature_change_range.1
        } else {
            -daily_temperature_change_range.0
        };

    return base_temperature
        + base_temperature
            * if base_temperature > 0.0 {
                temperature_shift
            } else {
                -temperature_shift
            };
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
struct DayHour {
    day: u32,
    hour: u32,
}

fn generate_hourly_rain_for_year(
    days: u32,
    rng: &mut RngComponent,
) -> HashMap<DayHour, RainIntensity> {
    let max_hours = 24 * 3;
    let mut until_index = 0;
    let mut intensity = 0.0;

    (0..days * 24)
        .map(|hour| {
            let rain_chance = 0.2;
            let rain_free_chance = 1.0 - rain_chance;

            if until_index <= hour {
                let current_rain_random: f32 = rng.f32_normalized();
                until_index = hour + rng.u32(0..max_hours);
                intensity = if current_rain_random > rain_free_chance {
                    (current_rain_random - rain_free_chance) * (1.0 / rain_chance)
                } else {
                    0.0
                }
            }

            return (
                DayHour {
                    day: hour / 24,
                    hour: hour % 24,
                },
                RainIntensity(intensity),
            );
        })
        .collect()
}
