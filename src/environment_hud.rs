use crate::datetime::GameTime;
use bevy::{
    prelude::{
        default, App, BuildChildren, Color, Commands, Component, IntoSystemAppConfig,
        IntoSystemConfigs, Label, NodeBundle, OnEnter, OnUpdate, Plugin, Query, Res, TextBundle,
    },
    text::{Text, TextStyle},
    ui::{JustifyContent, Size, Style, UiRect, Val},
};

use crate::{loading::FontAssets, GameState};
use sun_times::altitude;

#[derive(Component)]
struct DateTimeDisplay;

#[derive(Component)]
struct SunTracker(u32);
#[derive(Component)]
pub struct SunAltitude(pub f32);

pub struct EnvironmentHudPlugin;

impl Plugin for EnvironmentHudPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_environment_hud.in_schedule(OnEnter(GameState::Playing)))
            .add_systems((update_date_time_display,).in_set(OnUpdate(GameState::Playing)));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn sun_altitude_at_point(date_time: DateTime<Utc>) -> f32 {
    altitude(date_time, 51.527178, -0.109798) as f32 / 90.0
}

use chrono::{DateTime, Timelike, Utc};
fn update_date_time_display(
    mut tooltips: Query<(&mut Text, &mut SunTracker, &mut SunAltitude)>,
    game_time: Res<GameTime>,
) {
    let (mut tooltip, mut sun_tracker, mut sun_altitude) = tooltips.single_mut();
    let time = game_time.0.time();
    let hour = time.hour();
    let minute = time.minute();
    let formatted_hour = if hour < 10 {
        " ".to_owned() + &hour.to_string()
    } else {
        hour.to_string()
    };

    let formatted_minute = if minute < 10 {
        "0".to_owned() + &minute.to_string()
    } else {
        minute.to_string()
    };

    let text = formatted_hour + ":" + &formatted_minute;
    tooltip.sections[0].value = text;

    let sun_tracker_step_minutes = 5.0;
    let next_sun_tracker_value =
        ((hour as f32 * 60.0 + minute as f32) / sun_tracker_step_minutes).round() as u32;

    if sun_tracker.0 != next_sun_tracker_value {
        // TODO: refactor int osome kind of a countdown to make it a little bit neater
        sun_tracker.0 = next_sun_tracker_value;
        sun_altitude.0 = sun_altitude_at_point(game_time.0);
    }
}

fn create_environment_hud(
    mut commands: Commands,
    fonts: Res<FontAssets>,
    game_time: Res<GameTime>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::width(Val::Px(200.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::width(Val::Percent(100.0)),
                                ..default()
                            },
                            background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // text
                            parent
                                .spawn((
                                    TextBundle::from_section(
                                        "",
                                        TextStyle {
                                            font: fonts.hack.clone(),
                                            font_size: 30.0,
                                            color: Color::WHITE,
                                        },
                                    )
                                    .with_style(Style {
                                        margin: UiRect::all(Val::Px(5.0)),
                                        ..default()
                                    }),
                                    // Because this is a distinct label widget and
                                    // not button/list item text, this is necessary
                                    // for accessibility to treat the text accordingly.
                                    Label,
                                ))
                                .insert(DateTimeDisplay)
                                .insert((
                                    SunTracker(0), // UTC is wrong as it should be the timezone of the location
                                    SunAltitude(sun_altitude_at_point(game_time.0)),
                                ));
                        });
                });
        });
}
