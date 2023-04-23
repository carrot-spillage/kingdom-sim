use bevy::{
    prelude::{
        default, App, Color, Commands, Component, IntoSystemAppConfig, IntoSystemConfig, OnEnter,
        OnUpdate, Plugin, Query, Res, Transform, Vec3, With,
    },
    text::{Text, Text2dBundle, TextAlignment, TextStyle},
    window::Window,
};
use chrono::Timelike;

use crate::{create_world::WorldParams, loading::FontAssets, GameState};

use super::GameTime;

#[derive(Component)]
pub struct TimeTooltip;

pub struct GameTimeUIPlugin;

impl Plugin for GameTimeUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_tooltip_text.in_set(OnUpdate(GameState::Playing)))
            .add_system(create_tooltip.in_schedule(OnEnter(GameState::Playing)));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

pub fn update_tooltip_text(
    mut tooltips: Query<&mut Text, With<TimeTooltip>>,
    game_time: Res<GameTime>,
    fonts: Res<FontAssets>,
) {
    // ?? switch to monospace font, center relative to the screen offset
    let mut tooltip = tooltips.single_mut();
    let time = game_time.0.time();
    let minute = time.minute();
    let formatted_minute = if minute < 9 {
        "0".to_owned() + &minute.to_string()
    } else {
        minute.to_string()
    };

    *tooltip = create_text(time.hour().to_string() + ":" + &formatted_minute, &fonts);
}

fn create_text<S>(text: S, fonts: &Res<FontAssets>) -> Text
where
    S: Into<String>,
{
    let text_style = TextStyle {
        font: fonts.fira_sans.clone(),
        font_size: 21.0,
        color: Color::RED,
    };
    let text_alignment = TextAlignment::Center;
    Text::from_section(text, text_style.clone()).with_alignment(text_alignment)
}

pub fn create_tooltip(mut commands: Commands, fonts: Res<FontAssets>, windows: Query<&Window>) {
    let window = windows.single();
    commands
        .spawn(Text2dBundle {
            text: create_text("", &fonts),
            transform: Transform {
                translation: Vec3::new(window.width() / 2.0, 100.0, 1000.0),
                ..Transform::default()
            },
            ..default()
        })
        .insert(TimeTooltip);
}
