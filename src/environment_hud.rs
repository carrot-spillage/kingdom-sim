use crate::{ambience::Temperature, datetime::GameTime};
use bevy::{
    prelude::{
        default, in_state, App, BuildChildren, Changed, Color, Commands, Component,
        IntoSystemConfigs, Label, NodeBundle, OnEnter, Plugin, Query, Res, TextBundle, Update,
        With,
    },
    text::{Text, TextStyle},
    ui::{AlignItems, FlexDirection, JustifyContent, Style, UiRect, Val},
};

use crate::{loading::FontAssets, GameState};

#[derive(Component)]
struct DateTimeDisplay;

#[derive(Component)]
struct TemperatureDisplay;

pub struct EnvironmentHudPlugin;

impl Plugin for EnvironmentHudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), create_environment_hud)
            .add_systems(
                Update,
                (update_date_time_display, update_temperature_display)
                    .run_if(in_state(GameState::Playing)),
            );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

use chrono::{Datelike, Timelike};
fn update_date_time_display(
    mut tooltips: Query<&mut Text, With<DateTimeDisplay>>,
    game_time: Res<GameTime>,
) {
    let mut tooltip = tooltips.single_mut();
    let date = game_time.0.date_naive();
    let month = date.month0() as usize;

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

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

    let text = months[month].to_owned() + " " + &formatted_hour + ":" + &formatted_minute;
    tooltip.sections[0].value = text;
}

fn update_temperature_display(
    mut tooltips: Query<&mut Text, With<TemperatureDisplay>>,
    temperature_q: Query<&Temperature, Changed<Temperature>>,
) {
    if let Ok(Temperature(temperature)) = temperature_q.get_single() {
        let mut tooltip = tooltips.single_mut();

        let text = format!("{:.0}°C", temperature);
        tooltip.sections[0].value = text;
    }
}

fn create_environment_hud(mut commands: Commands, fonts: Res<FontAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(260.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            // left vertical fill (content)
            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                    ..default()
                })
                .with_children(|builder| {
                    // text

                    builder
                        .spawn(NodeBundle {
                            style: Style {
                                padding: UiRect {
                                    top: Val::Px(1.),
                                    left: Val::Px(5.),
                                    right: Val::Px(5.),
                                    bottom: Val::Px(1.),
                                },
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|builder| {
                            builder
                                .spawn(
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
                                )
                                .insert(TemperatureDisplay);
                        });

                    builder
                        .spawn(NodeBundle {
                            style: Style {
                                padding: UiRect {
                                    top: Val::Px(1.),
                                    left: Val::Px(5.),
                                    right: Val::Px(5.),
                                    bottom: Val::Px(1.),
                                },
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|builder| {
                            builder
                                .spawn(
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
                                )
                                .insert(DateTimeDisplay);
                        });
                });
        });
}
