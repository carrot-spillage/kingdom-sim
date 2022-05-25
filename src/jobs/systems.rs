use std::{any::Any, collections::HashMap};

use bevy::{
    math::{Vec2, Vec3},
    prelude::{
        App, Commands, Component, Entity, EventReader, EventWriter,
        ParallelSystemDescriptorCoercion, Plugin, Query, Res, ResMut, SystemSet, With, Without,
    },
};

use crate::{
    activity_info::ActivityInfo,
    init::{get_random_pos, WorldParams},
    jobs::helpers::{create_work_process, join_work_process},
    movement::{ArrivalEvent, MovingToPosition},
    GameState,
};

use super::{
    helpers::match_workers_with_jobs,
    work_process::{advance_work_process_state, SkillType, Skilled, WorkProcessState},
    AssignedToWorkProcess, JobQueue, WorkProcess,
};

pub fn add_work_systems(app: &mut App) {
    app.add_event::<WorkScheduledEvent>()
        .add_event::<WorkProgressedEvent>()
        .add_event::<WorkCompletedEvent>()
        .add_event::<DespawnWorkProccessEvent>()
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(despawn_completed_processes)
                .with_system(mark_processes_despawnable.after(despawn_completed_processes))
                .with_system(advance_all_work_processes.after(mark_processes_despawnable))
                .with_system(assign_jobs_to_workers.after(advance_all_work_processes))
                .with_system(handle_arrivals),
        );
}

fn assign_jobs_to_workers(
    // this should be the brain of work assignment. it should be in its own module
    mut commands: Commands,
    mut job_queue: ResMut<JobQueue>,
    world_params: Res<WorldParams>,
    workers_looking_for_jobs: Query<(Entity, &Skilled), Without<AssignedToWorkProcess>>,
    mut available_work_processess: Query<(Entity, &mut WorkProcess)>,
    mut activities: Query<&mut ActivityInfo>,
    mut work_scheduled_events: EventWriter<WorkScheduledEvent>,
) {
    let all_workers = workers_looking_for_jobs
        .iter()
        .map(|(entity, s)| (entity, s.clone()))
        .collect::<Vec<_>>();

    let worker_and_jobs = match_workers_with_jobs(&all_workers, &mut job_queue);
    for (worker_id, job) in worker_and_jobs {
        let maybe_existing_work_process =
            available_work_processess
                .iter_mut()
                .find(|(_, work_process)| {
                    work_process.max_workers
                        > (work_process.worker_ids.len() + work_process.tentative_worker_ids.len())
                        && job.id == work_process.job_id
                });

        let (work_process_id, position) = match maybe_existing_work_process {
            Some((work_process_id, mut work_process)) => {
                *work_process = join_work_process(&work_process, worker_id);
                (work_process_id, work_process.position)
            }
            None => {
                // big TODO: here should be some kind of AI to decide where to start the work process
                let position = get_random_pos(Vec2::ZERO, world_params.size / 2.0 - 300.0);
                let mut new_work_process = WorkProcess::new(position, job.id, 20.0, 2);
                new_work_process.tentative_worker_ids.push(worker_id);
                let work_process_id = commands.spawn().insert(new_work_process).id();

                // TODO: maybe we need to refactor .send() away from here

                work_scheduled_events.send(WorkScheduledEvent {
                    job_id: job.id,
                    position,
                    work_process_id,
                });

                (work_process_id, position)
            }
        };

        println!(
            "AssignedToWorkProcess is added and moving to {:?}",
            position
        );
        commands
            .entity(worker_id)
            .insert(AssignedToWorkProcess { work_process_id })
            .insert(MovingToPosition {
                position,
                sufficient_range: 30.0,
            });

        let mut activity = activities.get_mut(worker_id).unwrap();
        (*activity).title = format!("Moving to do '{job_name}'", job_name = job.name);
    }
}

fn handle_arrivals(
    mut arriveds: EventReader<ArrivalEvent>,
    mut assigned_workers: Query<(Entity, Option<&AssignedToWorkProcess>, &mut ActivityInfo)>,
    mut work_processes: Query<&mut WorkProcess>,
) {
    for ArrivalEvent(entity_id) in arriveds.iter() {
        let (worker_id, maybe_assigned, mut activity) =
            assigned_workers.get_mut(*entity_id).unwrap();
        match maybe_assigned {
            Some(AssignedToWorkProcess { work_process_id }) => {
                let mut work_process = work_processes.get_mut(*work_process_id).unwrap();
                (*work_process)
                    .tentative_worker_ids
                    .retain(|x| *x != worker_id);
                (*work_process).worker_ids.push(worker_id);
                println!("Working {:?}", worker_id);
                (*activity).title = "Working".to_string();
            }
            None => {
                println!("Idling {:?}", worker_id);

                (*activity).title = "Idling".to_string();
            }
        }
    }
}

/**
 * This is just to keep work_process alive for 1 more frame after it has been completed.
 * It is to prevent external systems from using it after it is despawned.
 */
fn mark_processes_despawnable(
    mut commands: Commands,
    mut completed_events: EventReader<WorkCompletedEvent>,
    mut despawn_events: EventWriter<DespawnWorkProccessEvent>,
) {
    for event in completed_events.iter() {
        despawn_events.send(DespawnWorkProccessEvent(event.work_process_id));
    }
}

fn despawn_completed_processes(
    mut commands: Commands,
    mut despawn_events: EventReader<DespawnWorkProccessEvent>,
) {
    for event in despawn_events.iter() {
        commands.entity(event.0).despawn();
    }
}

pub(crate) fn advance_all_work_processes(
    mut commands: Commands,
    mut work_processes: Query<(Entity, &mut WorkProcess)>,
    workers: Query<&Skilled>,
    job_queue: Res<JobQueue>,
    mut activities: Query<&mut ActivityInfo>,
    mut work_progressed_events: EventWriter<WorkProgressedEvent>,
    mut work_completed_events: EventWriter<WorkCompletedEvent>,
) {
    let occupied_work_processes = work_processes
        .iter_mut()
        .filter(|(_, wp)| wp.worker_ids.len() > 0);

    for (work_process_id, mut work_process) in occupied_work_processes {
        let workers: Vec<&Skilled> = work_process
            .worker_ids
            .iter()
            .map(|worker_id| workers.get(*worker_id).unwrap())
            .collect();
        let job = job_queue
            .jobs
            .iter()
            .find(|j| j.id == work_process.job_id)
            .unwrap();

        match advance_work_process_state(workers, &work_process.progress, job.skill_type) {
            WorkProcessState::CompleteWorkProcessState { quality } => {
                for worker_id in work_process
                    .worker_ids
                    .iter()
                    .chain(work_process.tentative_worker_ids.iter())
                {
                    commands
                        .entity(*worker_id)
                        .remove::<AssignedToWorkProcess>();

                    commands.entity(work_process_id).remove::<WorkProcess>(); // make it inaccessible for any the WP systems

                    let mut activity = activities.get_mut(*worker_id).unwrap();
                    (*activity).title = "Not AssignedToWorkProcess".to_string();

                    work_completed_events.send(WorkCompletedEvent {
                        job_id: work_process.job_id,
                        work_process_id,
                        worker_id: *worker_id,
                        quality,
                    });
                }
            }
            WorkProcessState::IncompleteWorkProcessState(progress) => {
                work_progressed_events.send(WorkProgressedEvent {
                    job_id: work_process.job_id,
                    work_process_id,
                    units_of_work: work_process.units_of_work,
                    units_of_work_left: progress.units_of_work_left,
                });
                (*work_process).progress = progress;
            }
        }
    }
}

pub struct WorkScheduledEvent {
    pub job_id: &'static str,
    pub position: Vec3,
    pub work_process_id: Entity,
}

pub struct WorkProgressedEvent {
    pub job_id: &'static str,
    pub work_process_id: Entity,
    pub units_of_work: f32,
    pub units_of_work_left: f32,
}

pub struct WorkCompletedEvent {
    pub job_id: &'static str,
    pub work_process_id: Entity,
    pub worker_id: Entity,
    pub quality: f32,
}

struct DespawnWorkProccessEvent(pub Entity);

#[derive(Clone, Copy)]
pub struct Job {
    pub id: &'static str,
    pub name: &'static str,
    pub skill_type: SkillType,
}

impl Job {
    pub fn new(name: &'static str, skill_type: SkillType) -> Self {
        Self {
            id: name, // TODO: maybe we just always use name, but for clarity I'm using id
            name,
            skill_type,
        }
    }
}
