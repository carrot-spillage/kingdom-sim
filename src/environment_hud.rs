use crate::datetime::GameTime;
use bevy::{
    prelude::{
        default, App, BuildChildren, Color, Commands, Component, Entity, IntoSystemAppConfig,
        IntoSystemConfig, IntoSystemConfigs, Label, NodeBundle, OnEnter, OnUpdate, Plugin, Query,
        Res, TextBundle, With,
    },
    text::{Text, TextStyle},
    ui::{JustifyContent, Size, Style, UiRect, Val},
};

use crate::{loading::FontAssets, GameState};
use sun_times::altitude;

#[derive(Component)]
struct DateTimeDisplay;

#[derive(Component)]
struct GameHour(u32);
#[derive(Component)]
struct SunAltitude(f32);

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

fn altitude_at_point(date_time: DateTime<Utc>) -> f32 {
    altitude(date_time, 41.0, 26.0) as f32
}

use chrono::{DateTime, Timelike, Utc};
fn update_date_time_display(
    mut tooltips: Query<(&mut Text, &mut GameHour, &mut SunAltitude), With<DateTimeDisplay>>,
    game_time: Res<GameTime>,
) {
    let (mut tooltip, mut game_hour, mut sun_altitude) = tooltips.single_mut();
    let time = game_time.0.time();
    let hour = time.hour();
    let minute = time.minute();
    let formatted_hour = if hour < 9 {
        " ".to_owned() + &hour.to_string()
    } else {
        hour.to_string()
    };

    let formatted_minute = if minute < 9 {
        "0".to_owned() + &minute.to_string()
    } else {
        minute.to_string()
    };

    let text = formatted_hour + ":" + &formatted_minute;
    tooltip.sections[0].value = text;

    if game_hour.0 != hour {
        // updating hour to be used for updating the Sun
        game_hour.0 = hour;
        sun_altitude.0 = altitude_at_point(game_time.0);
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
                                    GameHour(game_time.0.hour()), // UTC is wrong as it should be the timezone of the location
                                    SunAltitude(altitude_at_point(game_time.0)),
                                ));
                        });
                });
        });
}
