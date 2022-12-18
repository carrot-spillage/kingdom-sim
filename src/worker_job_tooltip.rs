use bevy::{
    math::Vec3,
    prelude::{
        default, App, Changed, Color, Component, Entity, EventReader, Plugin, Query, Res,
        SystemSet, Transform, Added,
    },
    text::{HorizontalAlign, Text, Text2dBundle, TextAlignment, TextStyle, VerticalAlign},
};

use crate::{
    loading::FontAssets,
    planned_work::{WorkerCompletedWorkEvent, WorkerStartedWorkEvent, WorkingOn, NotWorking},
    GameState,
};

#[derive(Component)]
pub struct WorkerJobTooltip {
    pub title: String,
    pub child: Entity,
}

pub struct WorkerJobTooltipPlugin;

impl Plugin for WorkerJobTooltipPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(update_work_tooltip_text).with_system(update_title));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn update_work_tooltip_text(
    job_labels: Query<&WorkerJobTooltip, Changed<WorkerJobTooltip>>,
    mut texts: Query<&mut Text>,
    fonts: Res<FontAssets>,
) {
    for worker_job_tooltip in job_labels.iter() {
        let mut text = texts.get_mut(worker_job_tooltip.child).unwrap();
        // println!("tracking activity {:?}", activity.title);
        *text = create_text(&worker_job_tooltip.title, &fonts);
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

fn update_title_2(
    mut work_started_events: EventReader<WorkerStartedWorkEvent>,
    mut work_completed_events: EventReader<WorkerCompletedWorkEvent>,
    mut worker_job_tooltip: Query<&mut WorkerJobTooltip>,
) {
    for event in work_started_events.iter() {
        let job_id = event.job_id;
        let mut worker_job_tooltip = worker_job_tooltip.get_mut(event.worker_id).unwrap();
        (*worker_job_tooltip).title = format!("Working: {job_id}");
    }

    for event in work_completed_events.iter() {
        let mut worker_job_tooltip = worker_job_tooltip.get_mut(event.worker_id).unwrap();
        (*worker_job_tooltip).title = format!("");
    }
}


fn update_title(
    started_working: Query<(Entity, &WorkingOn), Added<WorkingOn>>,
    stopped_working: Query<Entity, Added<NotWorking>>,
    mut worker_job_tooltip: Query<&mut WorkerJobTooltip>,
) {
    for (worker_id, works_on) in started_working.iter() {
        let mut worker_job_tooltip = worker_job_tooltip.get_mut(worker_id).unwrap();
        let job_id = works_on.job_id;
        (*worker_job_tooltip).title = format!("Working: {job_id}");
    }

    for worker_id in stopped_working.iter() {
        let mut worker_job_tooltip = worker_job_tooltip.get_mut(worker_id).unwrap();
        (*worker_job_tooltip).title = format!("");
    }
}