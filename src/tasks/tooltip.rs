use bevy::{
    math::Vec3,
    prelude::{
        default, Added, Changed, Color, Component, Entity, Query, Res, Transform, With, Without,
    },
    text::{Text, Text2dBundle, TextAlignment, TextStyle},
};

use crate::{
    loading::FontAssets,
    tasks::{CreatureTaskType, IdlingCreature},
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
    let text_alignment = TextAlignment::Center;
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
    mut task_completed_query: Query<
        &mut CreatureTaskTooltip,
        (Without<CreatureTaskType>, Added<IdlingCreature>),
    >,
    mut task_started_query: Query<
        (&mut CreatureTaskTooltip, &CreatureTaskType),
        (With<CreatureTaskType>, Added<CreatureTaskType>),
    >,
) {
    for (mut tootltip, task_type) in &mut task_started_query {
        let task_name = match task_type {
            CreatureTaskType::Plant { .. } => "Planting",
            CreatureTaskType::CutTree { .. } => "Cutting tree",
            CreatureTaskType::Harvest { .. } => "Harvesting",
            CreatureTaskType::MoveToTarget { .. } => "Moving to target",
            CreatureTaskType::MoveToPosition { .. } => "Moving to position",
            CreatureTaskType::DropItems { .. } => "Dropping items",
            CreatureTaskType::CollectItems { .. } => "Collecting items",
            CreatureTaskType::TransferItems { .. } => "Transferring items",
            CreatureTaskType::Build { .. } => "Building",
        };
        tootltip.title = format!("Task: {task_name}");
    }

    for mut tootltip in &mut task_completed_query {
        tootltip.title = format!("Idling");
    }
}
