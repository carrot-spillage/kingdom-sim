use bevy::{
    math::Vec3,
    prelude::{App, Commands, Entity, EventReader, EventWriter, Plugin, Query, SystemSet},
};

use crate::{
    building::{
        convert_construction_site_to_building, get_construction_site_texture,
        spawn_construction_site, BuildingBlueprint,
    },
    movement::Position,
    planned_work::{ArrivedToWorkEvent, PlannedWork, WorkerCompletedWorkEvent, BUILDING_JOB_NAME},
    skills::{SkillType, Skilled},
    work_progress::{advance_work_process_state, WorkProgress, WorkProgressUpdate},
    GameState,
};

pub struct BuildingJobPlugin;

impl Plugin for BuildingJobPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(handle_arrived_to_work)
                .with_system(handle_work_process),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn handle_arrived_to_work(
    mut work_ref_events: EventReader<ArrivedToWorkEvent>,
    mut work_query: Query<&mut PlannedWork>,
) {
    for work_id in work_ref_events
        .iter()
        .filter(|x| x.0.job_id == BUILDING_JOB_NAME)
        .map(|x| x.0.work_id)
    {
        let mut work = work_query.get_mut(work_id).unwrap();
    }
}

fn handle_work_process(
    mut commands: Commands,
    mut construction_sites: Query<(
        Entity,
        &PlannedWork,
        &Position,
        &mut WorkProgress,
        &BuildingBlueprint,
    )>,
    workers: Query<&Skilled>,
    mut worker_completion_events: EventWriter<WorkerCompletedWorkEvent>,
) {
    for (planned_work_id, work, position, mut work_progress, building_blueprint) in
        construction_sites.iter_mut()
    {
        let building_id = planned_work_id; // building is the planned work

        let workers: Vec<&Skilled> = work
            .worker_ids
            .iter()
            .map(|worker_id| workers.get(*worker_id).unwrap())
            .collect();

        if workers.is_empty() {
            continue;
        }

        if work_progress.units_of_work_left == work.units_of_work {
            spawn_construction_site(
                &mut commands,
                building_id,
                position.0,
                &building_blueprint.texture_set,
            );
        }

        match advance_work_process_state(workers, &work_progress, SkillType::Building) {
            WorkProgressUpdate::Complete { .. } => {
                for worker_id in work
                    .worker_ids
                    .iter()
                    .chain(work.tentative_worker_ids.iter())
                {
                    remove_planned_work(&mut commands, planned_work_id);

                    worker_completion_events.send(WorkerCompletedWorkEvent {
                        worker_id: *worker_id,
                    })
                }

                convert_construction_site_to_building(
                    building_id,
                    &mut commands,
                    &building_blueprint.texture_set,
                );
            }
            WorkProgressUpdate::Incomplete { progress, delta } => {
                if let Some(new_texture) = get_construction_site_texture(
                    1.0 - (progress.units_of_work_left + delta) / work.units_of_work,
                    1.0 - progress.units_of_work_left / work.units_of_work,
                    building_blueprint,
                ) {
                    commands.entity(building_id).insert(new_texture);
                }

                *work_progress = progress;
            }
        }
    }
}

pub fn plan_building(
    commands: &mut Commands,
    building_blueprint: BuildingBlueprint,
    position: Vec3,
) -> Entity {
    commands
        .spawn()
        .insert(PlannedWork::new(
            BUILDING_JOB_NAME,
            building_blueprint.units_of_work,
            building_blueprint.max_workers,
        ))
        .insert(WorkProgress::new(building_blueprint.units_of_work))
        .insert(Position(position))
        .insert(building_blueprint)
        .id()
}

fn remove_planned_work(commands: &mut Commands, planned_work_id: Entity) {
    commands
        .entity(planned_work_id)
        .remove::<WorkProgress>()
        .remove::<PlannedWork>();
}
