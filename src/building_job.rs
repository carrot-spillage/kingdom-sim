use bevy::prelude::{
    App, AssetServer, Commands, EventReader, Handle, Image, Plugin, Query, Res, SystemSet, With,
};

use crate::{
    building::{convert_construction_site_to_building, update_construction_site, ConstructionSite},
    common::CreationProgress,
    jobs::{
        work_process::SkillType, Job, JobQueue, WorkCompletedEvent, WorkProcess,
        WorkProgressedEvent,
    },
    GameState,
};

pub struct BuildingJobPlugin;

fn register_job(app: &mut App, job: Job) {
    app.world.get_resource_mut::<JobQueue>().unwrap().add(job);
}

impl Plugin for BuildingJobPlugin {
    fn build(&self, app: &mut App) {
        register_job(app, Job::new("Building", SkillType::Building));

        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(handle_work_progressed)
                .with_system(handle_work_completed),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn handle_work_progressed(
    mut events: EventReader<WorkProgressedEvent>,
    mut construction_progresses: Query<(&mut CreationProgress, &mut Handle<Image>)>,
    asset_server: Res<AssetServer>,
) {
    for progress_event in events.iter() {
        update_construction_site(progress_event, &mut construction_progresses, &asset_server);
    }
}

fn handle_work_completed(
    mut commands: Commands,
    mut events: EventReader<WorkCompletedEvent>,
    asset_server: Res<AssetServer>,
) {
    for event in events.iter() {
        convert_construction_site_to_building(event.work_process_id, &mut commands, &asset_server);
    }
}
