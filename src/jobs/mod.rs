pub mod helpers;
pub mod work_process;

use itertools::Itertools;
use std::collections::HashMap;

use bevy::{
    math::Vec3,
    prelude::{
        App, Commands, Component, Entity, Plugin, Query, Res, ResMut, SystemSet, With, Without,
    },
};

use crate::{
    activity_info::ActivityInfo,
    init::{get_random_pos_in_world, WorldParams},
    movement::{Arrived, MovingToPosition, Position},
    GameState,
};

use self::{
    helpers::{create_work_process, join_work_process, match_workers_with_jobs},
    work_process::{advance_work_process_state, SkillType, Skilled, WorkProcessState},
};

pub struct JobsPlugin;

impl Plugin for JobsPlugin {
    fn build(&self, app: &mut App) {
        let jobs = vec![Job {
            id: 1,
            name: "PlantingCrops",
            skill_type: SkillType::PlantingCrops,
        }];

        let job_priorities = jobs.iter().map(|j| (j.id, 0.5)).collect();
        app.insert_resource(JobQueue::new(jobs.clone(), job_priorities));
        app.insert_resource(jobs);
        app.add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(assign_jobs_to_workers),
        )
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
    accumulated_value_per_job: HashMap<u32, f32>,
    pub job_priorities: HashMap<u32, f32>,
}

impl JobQueue {
    pub fn new(jobs: Vec<Job>, job_priorities: HashMap<u32, f32>) -> Self {
        let accumulated_value_per_job = jobs.iter().map(|j| (j.id, 0.0)).collect();
        JobQueue {
            jobs,
            counter: 0,
            accumulated_value_per_job,
            job_priorities,
        }
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

fn assign_jobs_to_workers(
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
                        > ((work_process.worker_ids.len() as u32)
                            + (work_process.tentative_worker_ids.len() as u32))
                        && job.id == work_process.job_id
                });

        let (work_process_id, position) = match maybe_existing_work_process {
            Some((work_process_id, mut work_process)) => {
                *work_process = join_work_process(&work_process, worker_id);
                (work_process_id, work_process.position)
            }
            None => {
                // big TODO: find a way to provide position
                let position = get_random_pos_in_world(&world_params).0;
                let new_work_process = create_work_process(worker_id, position, &job);
                let work_process_id = commands.spawn().insert(new_work_process).id();
                (work_process_id, position)
            }
        };

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
    mut commands: Commands,
    mut arriveds: Query<(Entity, Option<&AssignedToWorkProcess>, &mut ActivityInfo), With<Arrived>>,
    mut assigned_workers: Query<(Option<&AssignedToWorkProcess>, &mut ActivityInfo)>,
    mut work_processes: Query<&mut WorkProcess>,
) {
    for (worker_id, maybe_assigned, mut activity) in arriveds.iter_mut() {
        match maybe_assigned {
            Some(AssignedToWorkProcess { work_process_id }) => {
                let mut work_process = work_processes.get_mut(*work_process_id).unwrap();
                (*work_process)
                    .tentative_worker_ids
                    .retain(|x| *x != worker_id);
                (*work_process).worker_ids.push(worker_id);
                (*activity).title = "Working".to_string();
            }
            None => {
                (*activity).title = "Idling".to_string();
            }
        }
    }
}

fn advance_all_work_processes(
    mut commands: Commands,
    mut work_processes: Query<&mut WorkProcess>,
    workers: Query<&Skilled>,
    job_queue: Res<JobQueue>,
    mut activities: Query<&mut ActivityInfo>,
) {
    for mut work_process in work_processes.iter_mut() {
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

        match advance_work_process_state(workers, &work_process.state, job.skill_type) {
            WorkProcessState::CompleteWorkProcessState { quality } => {
                for worker_id in work_process
                    .worker_ids
                    .iter()
                    .chain(work_process.tentative_worker_ids.iter())
                {
                    commands
                        .entity(*worker_id)
                        .remove::<AssignedToWorkProcess>();
                    commands.entity(*worker_id).insert(NotAssignedToWorkProcess); // TODO: meybe we don't need this flag

                    let mut activity = activities.get_mut(*worker_id).unwrap();
                    (*activity).title = "NotAssignedToWorkProcess".to_string();
                }
            }
            incomplete_state => {
                (*work_process).state = incomplete_state;
            }
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct Job {
    pub id: u32,
    pub name: &'static str,
    pub skill_type: SkillType,
}

#[derive(Component)]
pub struct AssignedToWorkProcess {
    pub work_process_id: Entity,
}

#[derive(Component)]
pub struct NotAssignedToWorkProcess;

#[derive(Component, Clone)]
pub struct WorkProcess {
    pub units_of_work: f32,
    pub job_id: u32,
    pub max_workers: u32,

    pub state: WorkProcessState,
    pub worker_ids: Vec<Entity>,
    pub tentative_worker_ids: Vec<Entity>,
    pub position: Vec3,
}
