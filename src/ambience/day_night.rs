use std::ops::Range;

use bevy::prelude::{
    in_state, App, Commands, Component, IntoSystemConfigs, Mat4, OnEnter, Plugin, Query, Res,
    Update, Vec3, Vec4,
};
use chrono::{DateTime, Timelike, Utc};
use sun_times::altitude;

use crate::{create_world::WorldParams, datetime::GameTime, GameState};

#[derive(Component)]
pub struct DayNightColorDistortion(pub Vec3);

pub struct DayNightPlugin;

impl Plugin for DayNightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), init_sun)
            .add_systems(Update, update_sun.run_if(in_state(GameState::Playing)));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

static DUSK_SPAN: f32 = 0.1;
static SUNRISE_SPAN: f32 = 0.2;
static DUSK_RANGE: Range<f32> = 0.0..DUSK_SPAN;
static SUNRISE_RANGE: Range<f32> = DUSK_RANGE.end..(DUSK_RANGE.end + SUNRISE_SPAN);
static MAX_SUNRISE_DISTORTION: Vec3 = Vec3::new(0.3, 0.1, -0.3);
static MAX_DUSK_DISTORTION: Vec3 = Vec3::new(-0.5, -0.4, -0.3);

fn get_day_night_color_distortion(sun_position: f32) -> Vec3 {
    if sun_position < DUSK_RANGE.start {
        Vec3::ONE + MAX_DUSK_DISTORTION
    } else if SUNRISE_RANGE.contains(&sun_position) {
        let distortion_scale = 1.0 - (sun_position - SUNRISE_RANGE.start) / SUNRISE_SPAN;
        Vec3::ONE + (MAX_SUNRISE_DISTORTION * distortion_scale)
    } else if DUSK_RANGE.contains(&sun_position) {
        let distortion_scale = 1.0 - (sun_position - DUSK_RANGE.start) / DUSK_SPAN;
        Vec3::ONE
            + (((MAX_DUSK_DISTORTION * distortion_scale)
                + (MAX_SUNRISE_DISTORTION * (1.0 - distortion_scale)))
                / 2.0)
    } else {
        Vec3::ONE
    }
}

#[derive(Component)]
struct SunIntervalTracker(u32);

#[derive(Component)]
pub struct SunAltitude(pub f32);

fn init_sun(mut commands: Commands, game_time: Res<GameTime>, world_params: Res<WorldParams>) {
    let sun_altitude = sun_altitude_at_point(game_time.0);

    // let mat4 = Mat4::from_cols(
    //     Vec4::new(1.0, 0.0, 0.0, 0.0),
    //     Vec4::new(0.0, 1.15, 0.25, 0.5),
    //     Vec4::new(0.0, 0.0, 1.0, 0.0),
    //     Vec4::new(0.0, 1.0, 0.0, 1.0),
    // );

    let mut mat4 = Mat4::IDENTITY;
    // Skew X by 0.25 of Y
    mat4.x_axis += Vec4::new(1.0, 0.25, 0.0, 0.0);
    mat4.y_axis += Vec4::ZERO;

    commands.spawn((
        DayNightColorDistortion(get_day_night_color_distortion(sun_altitude)),
        SunIntervalTracker(0),
        SunAltitude(sun_altitude),
    ));
}

fn update_sun(
    game_time: Res<GameTime>,
    mut sun_params: Query<(
        &mut SunAltitude,
        &mut SunIntervalTracker,
        &mut DayNightColorDistortion,
    )>,
) {
    let (mut sun_altitude, mut sun_tracking_countdown, mut day_night_color_distortion) =
        sun_params.single_mut();
    let time = game_time.0.time();
    let hour = time.hour();
    let minute = time.minute();
    let sun_tracker_step_minutes = 5.0;
    let next_sun_interval_tracker_value =
        ((hour as f32 * 60.0 + minute as f32) / sun_tracker_step_minutes).round() as u32;

    if sun_tracking_countdown.0 != next_sun_interval_tracker_value {
        // TODO: refactor int osome kind of a countdown to make it a little bit neater
        sun_tracking_countdown.0 = next_sun_interval_tracker_value;
        sun_altitude.0 = sun_altitude_at_point(game_time.0);
        day_night_color_distortion.0 = get_day_night_color_distortion(sun_altitude.0);
    }
}

fn sun_altitude_at_point(date_time: DateTime<Utc>) -> f32 {
    altitude(date_time, 51.527178, -0.109798) as f32 / 90.0
}
