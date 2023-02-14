use bevy::{
    math::Vec3,
    prelude::{
        default, Added, Changed, Color, Component, Entity, Query, Res, Transform, With, Without,
    },
    text::{HorizontalAlign, Text, Text2dBundle, TextAlignment, TextStyle, VerticalAlign},
};

use crate::{
    loading::FontAssets,
    tasks::{CreatureTask, IdlingCreature},
};

#[derive(Component)]
pub struct CreatureTaskTooltip {
    pub title: String,
    pub child: Entity,
}

pub fn update_tooltip_text(
    task_tooltips: Query<&CreatureTaskTooltip, Changed<CreatureTaskTooltip>>,
    mut texts: Query<&mut Text>,
    fonts: Res<FontAssets>,
) {
    for task_tooltip in task_tooltips.iter() {
        let mut text = texts.get_mut(task_tooltip.child).unwrap();
        // println!("tracking activity {:?}", activity.title);
        *text = create_text(&task_tooltip.title, &fonts);
    }
}

fn create_text<S>(text: S, fonts: &Res<FontAssets>) -> Text
where
    S: Into<String>,
{
    let text_style = TextStyle {
        font: fonts.fira_sans.clone(),
        font_size: 11.0,
        color: Color::ORANGE_RED,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    Text::from_section(text, text_style.clone()).with_alignment(text_alignment)
}

pub fn create_tooltip_bundle(top: f32, fonts: &Res<FontAssets>) -> Text2dBundle {
    Text2dBundle {
        text: create_text("Dummy", fonts),
        transform: Transform {
            translation: Vec3::new(0.0, top, 0.0),
            ..Transform::default()
        },
        ..default()
    }
}

pub fn update_tooltip(
    mut work_completed_query: Query<
        &mut CreatureTaskTooltip,
        (Without<CreatureTask>, Added<IdlingCreature>),
    >,
    mut work_started_query: Query<
        (&mut CreatureTaskTooltip, &CreatureTask),
        (With<CreatureTask>, Added<CreatureTask>),
    >,
) {
    for (mut tootltip, task) in &mut work_started_query {
        let task_name = match task {
            CreatureTask::Plant { .. } => "Planting",
            CreatureTask::CutTree { .. } => "Cutting tree",
            CreatureTask::Harvest { .. } => "Harvesting",
            CreatureTask::MoveToTarget { .. } => "Moving to target",
            CreatureTask::MoveToPosition { .. } => "Moving to position",
            CreatureTask::DropItems { .. } => "Dropping items",
        };
        tootltip.title = format!("Task: {task_name}");
    }

    for mut tootltip in &mut work_completed_query {
        tootltip.title = format!("Idling");
    }
}
