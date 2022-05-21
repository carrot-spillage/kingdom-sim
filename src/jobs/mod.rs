pub mod helpers;
pub mod work_process;

use itertools::Itertools;
use std::{any::Any, collections::HashMap};

use bevy::{
    math::Vec3,
    prelude::{
        App, Commands, Component, Entity, EventReader, EventWriter, Plugin, Query, Res, ResMut,
        SystemSet, With, Without,
    },
};

use crate::{
    activity_info::ActivityInfo,
    init::{get_random_pos_in_world, WorldParams},
    movement::{ArrivalEvent, MovingToPosition},
    GameState,
};

use self::{
    helpers::{create_work_process, join_work_process, match_workers_with_jobs},
    work_process::{
        advance_work_process_state, SkillType, Skilled, WorkProcessState, WorkProgress,
    },
};

pub struct JobsPlugin;

impl Plugin for JobsPlugin {
    fn build(&self, app: &mut App) {
        let jobs = vec![Job::new("PlantingCrops", SkillType::PlantingCrops)];

        let job_priorities = jobs.iter().map(|j| (j.id, 0.5)).collect();
        app.insert_resource(JobQueue::new(jobs.clone(), job_priorities))
            .insert_resource(jobs)
            .add_event::<WorkCompletedEvent>()
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(assign_jobs_to_workers),
            )
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(handle_arrivals))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(advance_all_work_processes),
            );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

pub struct JobQueue {
    pub jobs: Vec<Job>,
    counter: usize,
    accumulated_value_per_job: HashMap<usize, f32>,
    pub job_priorities: HashMap<usize, f32>,
}

impl JobQueue {
    pub fn new(jobs: Vec<Job>, job_priorities: HashMap<usize, f32>) -> Self {
        let accumulated_value_per_job = jobs.iter().map(|j| (j.id, 0.0)).collect();
        JobQueue {
            jobs,
            counter: 0,
            accumulated_value_per_job,
            job_priorities,
        }
    }

    pub fn add(&mut self, job: Job) {
        self.jobs.push(job);
        self.accumulated_value_per_job.insert(job.id, 0.0);
    }

    pub fn next(&mut self) -> Job {
        loop {
            let job = self.jobs[self.counter];

            let mut acc_value = self.accumulated_value_per_job.get(&job.id).unwrap()
                + self.job_priorities.get(&job.id).unwrap();
            if acc_value >= 1.0 {
                acc_value -= 1.0;
                self.accumulated_value_per_job.insert(job.id, acc_value);
                return job;
            }

            self.accumulated_value_per_job.insert(job.id, acc_value);

            self.counter += 1;

            if self.counter >= self.jobs.len() {
                self.counter = 0;
            }
        }
    }
}

fn assign_jobs_to_workers( // this should be the brain of work assignment. it should be in its own module
    mut commands: Commands,
    mut job_queue: ResMut<JobQueue>,
    world_params: Res<WorldParams>,
    workers_looking_for_jobs: Query<(Entity, &Skilled), Without<AssignedToWorkProcess>>,
    mut available_work_processess: Query<(Entity, &mut WorkProcess)>,
    mut activities: Query<&mut ActivityInfo>,
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
                let position = get_random_pos_in_world(&world_params).0;
                let new_work_process = create_work_process(worker_id, position, &job);
                let work_process_id = commands.spawn().insert(new_work_process).id();
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
        (*activity).title = format!("Moving to {job_name}", job_name = job.name);
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

fn advance_all_work_processes(
    mut commands: Commands,
    mut work_processes: Query<(Entity, &mut WorkProcess)>,
    workers: Query<&Skilled>,
    job_queue: Res<JobQueue>,
    mut activities: Query<&mut ActivityInfo>,
    mut work_progressed_events: EventWriter<WorkProgressedEvent>,
    mut work_completed_events: EventWriter<WorkCompletedEvent>,
) {
    for (work_process_id, mut work_process) in work_processes.iter_mut() {
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
                    println!("AssignedToWorkProcess is removed");
                    commands
                        .entity(*worker_id)
                        .remove::<AssignedToWorkProcess>();

                    let mut activity = activities.get_mut(*worker_id).unwrap();
                    (*activity).title = "Not AssignedToWorkProcess".to_string();

                    work_completed_events.send(WorkCompletedEvent {
                        job_id: work_process.job_id,
                        work_process_id,
                        worker_id: *worker_id,
                        quality,
                    });
                }

                commands.entity(work_process_id).despawn();
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

pub struct WorkProgressedEvent {
    pub job_id: usize,
    pub work_process_id: Entity,
    pub units_of_work: f32,
    pub units_of_work_left: f32,
}

pub struct WorkCompletedEvent {
    pub job_id: usize,
    pub work_process_id: Entity,
    pub worker_id: Entity,
    pub quality: f32,
}

use std::sync::atomic::{AtomicUsize, Ordering};
fn generate_job_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[derive(Clone, Copy)]
pub struct Job {
    pub id: usize,
    pub name: &'static str,
    pub skill_type: SkillType,
}

impl Job {
    pub fn new(name: &'static str, skill_type: SkillType) -> Self {
        Self {
            id: generate_job_id(),
            name,
            skill_type,
        }
    }
}

#[derive(Component)]
pub struct AssignedToWorkProcess {
    pub work_process_id: Entity,
}

#[derive(Component, Clone)]
pub struct WorkProcess {
    pub units_of_work: f32,
    pub job_id: usize,
    pub max_workers: usize,

    pub progress: WorkProgress,
    pub worker_ids: Vec<Entity>,
    pub tentative_worker_ids: Vec<Entity>,
    pub position: Vec3,
}
