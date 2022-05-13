use bevy::{
    math::Vec3,
    prelude::{
        default, App, AssetServer, Changed, Color, Component, Plugin, Query, Res, SystemSet,
        Transform,
    },
    text::{HorizontalAlign, Text, Text2dBundle, TextAlignment, TextStyle, VerticalAlign},
};

use crate::GameState;

fn create_text<S>(text: S, asset_server: &Res<AssetServer>) -> Text
where
    S: Into<String>,
{
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 11.0,
        color: Color::ORANGE_RED,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    Text::with_section(text, text_style.clone(), text_alignment)
}

pub fn create_activity_bundle(top: f32, asset_server: &Res<AssetServer>) -> Text2dBundle {
    Text2dBundle {
        text: create_text("Dummy", asset_server),
        transform: Transform {
            translation: Vec3::new(0.0, top, 0.0),
            ..Transform::default()
        },
        ..default()
    }
}

#[derive(Component)]
pub struct ActivityInfo(pub &'static str);

fn track_work_status(
    mut activities: Query<(&mut Text, &ActivityInfo), Changed<ActivityInfo>>,
    asset_server: Res<AssetServer>,
) {
    for (mut text, activity) in activities.iter_mut() {
        *text = create_text(activity.0, &asset_server);
    }
}

pub struct ActivityInfoPlugin;

impl Plugin for ActivityInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(track_work_status));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}