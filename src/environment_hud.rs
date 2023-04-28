use crate::datetime::GameTime;
use bevy::{
    prelude::{
        default, App, BuildChildren, Color, Commands, Component, IntoSystemAppConfig,
        IntoSystemConfig, Label, NodeBundle, OnEnter, OnUpdate, Plugin, Query, Res, TextBundle,
        With,
    },
    text::{Text, TextStyle},
    ui::{JustifyContent, Size, Style, UiRect, Val},
};

use crate::{loading::FontAssets, GameState};

#[derive(Component)]
pub struct TimeTooltip;

pub struct EnvironmentHudPlugin;

impl Plugin for EnvironmentHudPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_environment_hud.in_schedule(OnEnter(GameState::Playing)))
            .add_system(update.in_set(OnUpdate(GameState::Playing)));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

use chrono::Timelike;
pub fn update(mut tooltips: Query<&mut Text, With<TimeTooltip>>, game_time: Res<GameTime>) {
    let mut tooltip = tooltips.single_mut();
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
}

fn create_environment_hud(mut commands: Commands, fonts: Res<FontAssets>) {
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
                                .insert(TimeTooltip);
                        });
                });
        });
}
