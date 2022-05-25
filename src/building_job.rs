use bevy::prelude::{
    App, AssetServer, Commands, Component, Entity, EventReader, Handle, Image, Plugin, Query, Res,
    SystemSet,
};

use crate::{
    building::{
        convert_construction_site_to_building, spawn_construction_site, update_construction_site,
    },
    common::CreationProgress,
    jobs::{
        systems::{WorkScheduledEvent, WorkCompletedEvent, WorkProgressedEvent}, work_process::SkillType, JobQueue,
    },
    GameState,
};
use crate::jobs::systems::Job;

#[derive(Component)]
pub struct BuildingReference(pub Entity);

pub struct BuildingJobPlugin;

fn register_job(app: &mut App, job: Job) {
    app.world.get_resource_mut::<JobQueue>().unwrap().add(job);
}

static JOB_NAME: &'static str = "Building";

impl Plugin for BuildingJobPlugin {
    fn build(&self, app: &mut App) {
        register_job(app, Job::new(JOB_NAME, SkillType::Building));

        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(handle_work_scheduled)
                .with_system(handle_work_progressed)
                .with_system(handle_work_completed),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn handle_work_scheduled(
    mut commands: Commands,
    mut events: EventReader<WorkScheduledEvent>,
    asset_server: Res<AssetServer>,
) {
    for scheduled_event in events.iter().filter(|e| e.job_id == JOB_NAME) {
        let building_id =
            spawn_construction_site(&mut commands, scheduled_event.position, &asset_server);
        commands
            .entity(scheduled_event.work_process_id)
            .insert(BuildingReference(building_id));
    }
}

fn handle_work_progressed(
    mut events: EventReader<WorkProgressedEvent>,
    building_references: Query<&BuildingReference>,
    mut construction_progresses: Query<(&mut CreationProgress, &mut Handle<Image>)>,
    asset_server: Res<AssetServer>,
) {
    for progress_event in events.iter().filter(|e| e.job_id == JOB_NAME) {
        update_construction_site(
            progress_event,
            &building_references,
            &mut construction_progresses,
            &asset_server,
        );
    }
}

fn handle_work_completed(
    mut commands: Commands,
    mut events: EventReader<WorkCompletedEvent>,
    building_references: Query<&BuildingReference>,
    asset_server: Res<AssetServer>,
) {
    for event in events.iter().filter(|e| e.job_id == JOB_NAME) {
        let building_id = building_references.get(event.work_process_id).unwrap().0;
        convert_construction_site_to_building(building_id, &mut commands, &asset_server);
    }
}
