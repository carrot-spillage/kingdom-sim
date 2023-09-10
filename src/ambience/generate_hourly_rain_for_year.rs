use std::iter::repeat;

use bevy_turborand::{DelegatedRng, RngComponent};

#[derive(Debug, Clone, Copy)]
pub struct WeatherBlock {
    pub cloud_density: f32,
    pub rain_intensity: f32,
}

pub fn generate_hourly_rain_for_year(days: u32, rng: &mut RngComponent) -> Vec<WeatherBlock> {
    let max_hours: u32 = 24 * 3;
    let mut hours_left: u32 = days as u32 * 24;
    let mut weather_entries: Vec<WeatherBlock> = Vec::new();

    while hours_left > 0 {
        let period = std::cmp::min(
            hours_left,
            (rng.f32_normalized() * max_hours as f32).round() as u32,
        );
        let is_off = rng.f32_normalized() < 0.5;

        if is_off {
            let none = WeatherBlock {
                cloud_density: 0.0,
                rain_intensity: 0.0,
            };
            weather_entries.extend(repeat(none).take(period as usize));
        } else {
            let curve = calculate_cloud_curve(period, rng);
            weather_entries.append(&mut generate_period_data(&curve));
        }

        hours_left -= period;
    }

    weather_entries
}

fn generate_period_data(curve: &Curve) -> Vec<WeatherBlock> {
    (0..curve.length)
        .map(|i| {
            let density = get_cloud_density(
                i as f32,
                curve.length as f32,
                curve.max,
                curve.up_ratio,
                curve.down_ratio,
            );

            WeatherBlock {
                cloud_density: density,
                rain_intensity: density - curve.inner_offset.max(0.0),
            }
        })
        .collect()
}

#[derive(Debug)]
struct Curve {
    up_ratio: f32,
    down_ratio: f32,
    length: u32,
    max: f32,
    inner_offset: f32,
}

fn calculate_cloud_curve(length: u32, rng: &mut RngComponent) -> Curve {
    let outer_max = rng.f32_normalized();
    let down_start = rng.f32_normalized();
    let inner_offset = rng.f32_normalized() * outer_max.powi(3);

    Curve {
        up_ratio: down_start,
        down_ratio: 1.0 - down_start,
        length,
        max: outer_max,
        inner_offset,
    }
}

fn get_cloud_density(
    current_position: f32,
    length: f32,
    max: f32,
    up_spline_ratio: f32,
    down_spline_ratio: f32,
) -> f32 {
    if current_position > length {
        panic!("Current time cannot exceed transition duration");
    }

    let t = current_position / length;
    let spline_up = if t < up_spline_ratio {
        (max / (up_spline_ratio.powi(3))) * t.powi(3)
    } else {
        max
    };

    let spline_down = if t > 1.0 - down_spline_ratio {
        (max / (down_spline_ratio.powi(3))) * (1.0 - t).powi(3)
    } else {
        max
    };

    let density = spline_up * (1.0 - spline_down);
    density
}
